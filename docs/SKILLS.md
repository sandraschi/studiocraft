# SKILLS - StudioCraft v0.2 (6 guided recipes)

All skills run over `dispatch` (CLI, MCP, and GUI agree). See TOOLS.md.

1. **photo-vector-roundtrip** - the core safe loop (init/open -> check -> edit -> check -> open).
2. **safe-handoff-gate** - read allowed:false as a fix list (unsaved / unsupported / companion-target).
3. **companion-link** - attach design/pdf/word/deck/grid/effect/film/cad/sound as pointer JSON.
4. **rescue-blocked** - open -> re-check clean -> apply safe gate -> bump only when green.
5. **project-hygiene** - open first, no stray outputs inside, bump after verified trip, list to confirm.
6. **studio-chat-recipe** - caps + handoff JSON into system prompt, chat advises, gate decides.

Safety (all skills): never flatten, no export overwrites a native dir,
revision is checked_add, keys in memory/env only.
