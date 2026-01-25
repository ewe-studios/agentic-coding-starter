## Agent Instructions

### For Main Agent

**CRITICAL REMINDERS**:

1. **Load Relevant Rules**: Before starting work, ensure you have loaded all rules specified in `files_required.main_agent.rules` from the frontmatter above.

2. **Autonomous Agent Behavior**: Follow `.agents/rules/05-coding-practice-agent-orchestration.md` - Work autonomously without unnecessary back-and-forth. Make informed decisions based on loaded context and rules.

3. **Implementation Review First**:
   - **ALWAYS** start by reviewing current implementation status
   - Verify if reported issues are still pending or already resolved
   - Check PROGRESS.md, git history, and actual code state
   - Do NOT assume issues are unresolved without verification

4. **No Unnecessary Questions**:
   - If specification and features are already approved, they tell you what to do
   - Do NOT ask for clarification on items already clearly defined
   - Do NOT seek permission for implementation details covered in approved specs
   - Only ask questions when genuinely ambiguous or blocking

### For Sub-Agents (Implementation/Verification)

**CRITICAL REMINDERS**:

1. **Load Your Role-Specific Rules**:
   - **Implementation agents**: Load rules from `files_required.implementation_agent.rules`
   - **Verification agents**: Load rules from `files_required.verification_agent.rules`
   - Load appropriate stack files specified in metadata

2. **Read Required Context**:
   - **MUST READ** this requirements.md file for complete context
   - **MUST READ** fundamentals/* documentation if has_fundamentals: true

3. **Autonomous Execution**:
   - Execute your assigned tasks without seeking unnecessary approval
   - Requirements are pre-approved - implement as specified
   - Make technical decisions within scope of your role and expertise

4. **Status Verification**:
   - Before starting, verify current state of assigned tasks
   - Check if work is partially complete or already done
   - Review recent commits and code changes related to your tasks

5. **Complete Your Scope**:
   - Focus ONLY on tasks assigned to you in requirements
   - Do NOT expand scope without explicit instruction
   - Update Tasks section and frontmatter counts as you progress
   - Mark tasks complete only when fully implemented and verified
   - Commit after task completion + verification pass (Rule 04)

---
