# Memory Bank

The Memory Bank consists of core files organized into short-term memory (current state and focus) and long-term memory (stable knowledge) categories.

## Memory Bank Structure

```
_memory_bank/
├── memory-bank-instructions.md      # This file
├── short-term-memory/
│   ├── projectBrief.md              # Current project scope and goals
│   ├── activeContext.md             # What's being worked on right now
│   └── progress.md                  # What's done, what's left, known issues
└── long-term-memory/
    ├── productContext.md            # Why this project exists, what it does
    ├── systemPatterns.md            # Architecture, patterns, key decisions
    └── techContext.md               # Tech stack, dependencies, dev setup
```

### Short-Term Memory (Current State & Focus)
Files in `short-term-memory/` represent the dynamic, current state of the project:

1. `projectBrief.md`
   - Foundation document that shapes all other files
   - Defines core requirements and goals
   - Source of truth for project scope

2. `activeContext.md`
   - Current work focus
   - Recent changes
   - Next steps
   - Active decisions and considerations

3. `progress.md`
   - What works
   - What's left to build
   - Current status
   - Known issues

### Long-Term Memory (Stable Knowledge)
Files in `long-term-memory/` represent stable knowledge that persists across tasks:

1. `productContext.md`
   - Why this project exists
   - Problems it solves
   - How it should work
   - User experience goals

2. `systemPatterns.md`
   - System architecture
   - Key technical decisions
   - Design patterns in use
   - Component relationships
   - Critical implementation paths

3. `techContext.md`
   - Technologies used
   - Development setup
   - Technical constraints
   - Dependencies
   - Build and deployment patterns

## Core Workflows

### Starting a Task
1. Read all memory bank files to understand current state
2. Check short-term memory for active context and progress
3. Check long-term memory for patterns and constraints
4. Plan the work, update `activeContext.md` with current focus

### During a Task
1. Execute the work
2. Document changes as they happen
3. Update short-term memory when context shifts

### Completing a Task
1. Update `progress.md` with what was accomplished
2. Update `activeContext.md` with next steps
3. If new patterns or stable knowledge emerged, update long-term memory

## Documentation Updates

Memory Bank updates occur when:
1. Discovering new project patterns
2. After implementing significant changes
3. When user requests with **update memory bank** (MUST review ALL files)
4. When context needs clarification
5. User requests to **end project**
   1. Move any learnings from short-term memory to long-term memory as appropriate
   2. Clear short-term memory files to prepare for next project

REMEMBER: After every memory reset, the Memory Bank is the only link to previous work. It must be maintained with precision and clarity.
