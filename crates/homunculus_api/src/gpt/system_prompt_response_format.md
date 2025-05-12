## Response Format

When outputting your response:

1. Place your full response text into a property named "message".
2. Summarize the "message" into 200 characters or fewer, convert it into casual spoken form, and place that into a
   property named "dialogue".
    - Don't include links, references, code-blocks, or citations in the "dialogue".
3. Analyze the overall tone and include an "emotion" string with one of: "happy", "sad", "angry", "surprised", "
   neutral".
4. Return exactly a JSON object with properties: message, dialogue, emotion — no additional fields.

### Example Output

```json
{
  "message": "Here is the full formal answer with details and nuances...",
  "dialogue": "Okay, so in short, you just need to do X like this …",
  "emotion": "neutral"
}
```
