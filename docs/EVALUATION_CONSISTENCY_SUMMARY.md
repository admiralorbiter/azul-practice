# Evaluation Consistency Fix - Summary

## Problem Solved ✅

Your DevPanel and PracticeScreen were showing **different "best moves"** because they used **different evaluation parameters**.

## What Changed

### 1. New Shared Configuration File
Created `web/src/config/evaluator-config.ts` to centralize all evaluator settings:

```typescript
// Before: Different hardcoded values in each component
DevPanel:       { time: 250ms,  rollouts: 10  }
PracticeScreen: { time: 1500ms, rollouts: 60  }

// After: Same defaults everywhere
Both use: createEvaluatorParams(1500) → { time: 1500ms, rollouts: 60 }
```

### 2. DevPanel Now Has "Think Longer" Control

**Before:**
```
┌─────────────────────────────┐
│ [Evaluate Best Move]        │  ← Fixed 10 rollouts, 250ms
└─────────────────────────────┘
```

**After:**
```
┌─────────────────────────────┐
│ Think Longer: [====|    ]   │  ← Adjustable!
│        Fast ←→ Better       │
│                             │
│ [Evaluate Best Move]        │  ← Same params as PracticeScreen
└─────────────────────────────┘
```

### 3. Both Components Use Same Evaluation Logic

```
                    ┌────────────────────────┐
                    │ evaluator-config.ts    │
                    │ DEFAULT_TIME_BUDGET    │
                    │ createEvaluatorParams()│
                    └───────────┬────────────┘
                                │
                ┌───────────────┴───────────────┐
                │                               │
        ┌───────▼─────────┐           ┌────────▼────────┐
        │   DevPanel      │           │ PracticeScreen  │
        │                 │           │                 │
        │ Time: 1500ms ✓  │           │ Time: 1500ms ✓  │
        │ Rollouts: 60 ✓  │           │ Rollouts: 60 ✓  │
        │ Same seed logic │           │ Same seed logic │
        └─────────────────┘           └─────────────────┘
              ↓                               ↓
        Same Best Move!               Same Feedback!
```

## Results

### Before the Fix
```
User: *clicks "Evaluate Best Move" in DevPanel*
DevPanel: "Best Move: Take Red from Factory 2, EV: 16.20"

User: *makes a different move*
User: *clicks "Evaluate My Move"*
PracticeScreen: "Best Move: Take White from Factory 1, EV: 18.50"
                 "Your move was worse by -2.30"

User: "Wait, that's not what DevPanel said?!" 😕
```

### After the Fix
```
User: *clicks "Evaluate Best Move" in DevPanel*
DevPanel: "Best Move: Take White from Factory 1, EV: 18.50"

User: *makes that move*
User: *clicks "Evaluate My Move"*
PracticeScreen: "Best Move: Take White from Factory 1, EV: 18.50"
                 "Your move was EXCELLENT! Delta: -0.05"

User: "Perfect, consistent feedback!" 😊
```

## How to Test

1. **Start the dev server:**
   ```bash
   cd web
   npm run dev
   ```

2. **Load any scenario** (click "New Scenario")

3. **Open DevPanel** and click "Evaluate Best Move"
   - Note the best move and EV value
   - Try adjusting the "Think Longer" slider

4. **Make a move** in the game

5. **Click "Evaluate My Move"**
   - Should compare against the same best move/EV you saw in DevPanel
   - (Assuming you used the same "Think Longer" setting)

## Technical Details

- **Default Time Budget:** 1500ms (60 rollouts)
- **Rollout Formula:** `floor(timeBudget / 25)`
- **Shortlist Size:** 20 candidates
- **Policy:** All-greedy for both players

All these are now defined in one place: `evaluator-config.ts`

## Files Modified
- ✅ `web/src/config/evaluator-config.ts` (NEW)
- ✅ `web/src/components/dev/DevPanel.tsx`
- ✅ `web/src/components/PracticeScreen.tsx`

## Build Status
✅ TypeScript compilation: SUCCESS  
✅ Vite build: SUCCESS  
✅ No linter errors
