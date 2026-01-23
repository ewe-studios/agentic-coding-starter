# Claude API Usage in Artifacts

## Basic API Call Structure

```javascript
const response = await fetch("https://api.anthropic.com/v1/messages", {
  method: "POST",
  headers: {
    "Content-Type": "application/json",
  },
  body: JSON.stringify({
    model: "claude-sonnet-4-20250514",
    max_tokens: 1000,
    messages: [{ role: "user", content: "Your prompt here" }],
  }),
});
const data = await response.json();
```

Note: You don't need to pass in an API key - these are handled on the backend. You only need to pass in the messages array, max_tokens, and a model (which should always be claude-sonnet-4-20250514)

## API Response Structure

```javascript
// The response data will have this structure:
{
  content: [
    {
      type: "text",
      text: "Claude's response here"
    }
  ],
  // ... other fields
}

// To get Claude's text response:
const claudeResponse = data.content[0].text;
```

## Handling Images and PDFs

### PDF Handling

```javascript
// First, convert the PDF file to base64 using FileReader API
const base64Data = await new Promise((resolve, reject) => {
  const reader = new FileReader();
  reader.onload = () => {
    const base64 = reader.result.split(",")[1]; // Remove data URL prefix
    resolve(base64);
  };
  reader.onerror = () => reject(new Error("Failed to read file"));
  reader.readAsDataURL(file);
});

// Then use the base64 data in your API call
messages: [
  {
    role: "user",
    content: [
      {
        type: "document",
        source: {
          type: "base64",
          media_type: "application/pdf",
          data: base64Data,
        },
      },
      {
        type: "text",
        text: "What are the key findings in this document?",
      },
    ],
  },
];
```

### Image Handling

```javascript
messages: [
  {
    role: "user",
    content: [
      {
        type: "image",
        source: {
          type: "base64",
          media_type: "image/jpeg", // Make sure to use the actual image type here
          data: imageData, // Base64-encoded image data as string
        },
      },
      {
        type: "text",
        text: "Describe this image.",
      },
    ],
  },
];
```

## Structured JSON Responses

To ensure you receive structured JSON responses from Claude:

1. Specify the desired output format explicitly:
   "Respond only with a valid JSON object in the following format:"

2. Provide a sample JSON structure:

   ```json
   {
     "key1": "string",
     "key2": number,
     "key3": {
       "nestedKey1": "string",
       "nestedKey2": [1, 2, 3]
     }
   }
   ```

3. Use strict language: "Your entire response must be a single, valid JSON object. Do not include any text outside of the JSON structure, including backticks."

4. Be emphatic: "DO NOT OUTPUT ANYTHING OTHER THAN VALID JSON"

## Context Window Management

Since Claude has no memory between completions, you must include all relevant state information in each prompt.

### For Conversations:

```javascript
// Maintain an array of ALL previous messages
const conversationHistory = [
  { role: "user", content: "Hello, Claude!" },
  { role: "assistant", content: "Hello! How can I assist you today?" },
  // ... ALL previous messages should be included here
];

// Add the new user message
const newMessage = {
  role: "user",
  content: "Tell me more about machine learning.",
};

const response = await fetch("https://api.anthropic.com/v1/messages", {
  method: "POST",
  headers: {
    "Content-Type": "application/json",
  },
  body: JSON.stringify({
    model: "claude-sonnet-4-20250514",
    max_tokens: 1000,
    messages: [...conversationHistory, newMessage],
  }),
});
```

### For Stateful Applications:

```javascript
// Keep track of ALL relevant state
const gameState = {
  player: {
    name: "Hero",
    health: 80,
    inventory: ["sword", "health potion"],
    pastActions: ["Entered forest", "Fought goblin", "Found health potion"],
  },
  currentLocation: "Dark Forest",
  gameHistory: [
    { action: "Game started", result: "Player spawned in village" },
    // ... ALL relevant past events should be included here
  ],
};

// Include complete state in prompt
const prompt = `
Given the following COMPLETE game state and history:
${JSON.stringify(gameState, null, 2)}

The player's last action was: "Use health potion"

IMPORTANT: Consider the ENTIRE game state and history provided above when determining the result.
`;
```

## Error Handling

````javascript
try {
  const response = await fetch("https://api.anthropic.com/v1/messages", {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({
      model: "claude-sonnet-4-20250514",
      max_tokens: 1000,
      messages: [{ role: "user", content: prompt }],
    }),
  });

  if (!response.ok) {
    throw new Error(`API request failed: ${response.status}`);
  }

  const data = await response.json();

  // For regular text responses:
  const claudeResponse = data.content[0].text;

  // If expecting JSON response, parse it:
  if (expectingJSON) {
    // Handle Claude API JSON responses with markdown stripping
    let responseText = data.content[0].text;
    responseText = responseText
      .replace(/```json\n?/g, "")
      .replace(/```\n?/g, "")
      .trim();
    const jsonResponse = JSON.parse(responseText);
  }
} catch (error) {
  console.error("Error in Claude completion:", error);
  // Handle the error appropriately in your UI
}
````
