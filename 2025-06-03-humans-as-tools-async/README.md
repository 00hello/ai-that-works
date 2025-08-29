
# Humans as Tools: Async Agents and Durable Execution

[Video](https://youtu.be/NMhH5_ju3-I)

<a href="https://www.youtube.com/watch?v=NMhH5_ju3-I"><img width="600" alt="Screenshot 2025-06-10 at 8 56 45 AM" src="https://github.com/user-attachments/assets/1c01a45f-0103-43fd-98cd-4e2adb59c04f" /></a>

This session builds on our [12-factor agents workshop](../2025-04-22-twelve-factor-agents) to explore async agents and durable execution patterns. We'll learn how to build agents that can pause, contact humans for feedback or approval, and resume execution based on human responses.

## What You'll Learn

- How to implement async agent patterns with human-in-the-loop workflows
- State management for durable agent execution
- Different channels for human interaction (CLI, HTTP, email)
- Webhook integration for non-blocking human approvals
- Testing strategies for async agent workflows

## Key Takeaways

- Two types of human interaction - deterministic (code enforces human approval) and non-deterministic (agent chooses to contact a human)
- approver might not be the person interacting with the chatbot
- State management is key to building agents that can pause/resume for human interaction
- Separate concerns of inner loop (agent) and outer loop (human interaction)

## Whiteboards

### inner vs outer loop

![image](https://github.com/user-attachments/assets/3f3269f1-e177-473f-a4bc-7802255447dc)


### deterministic vs non-deterministic human approval

![image](https://github.com/user-attachments/assets/a36a19ec-52fa-43d1-be02-63cbf209d11e)


### base agent architecture refresh

![image](https://github.com/user-attachments/assets/b11a5c94-b1a0-4d02-89fb-9640ce436484)


![image](https://github.com/user-attachments/assets/661500e9-ba0e-496e-a774-e0add0d2b8e6)


![image](https://github.com/user-attachments/assets/d54415a4-5452-4035-8cf8-70b13ef3dafd)


## Running the Code

- Basic TypeScript knowledge
- Node.js 20+ installed
- Understanding of async/await patterns
- Familiarity with HTTP APIs and webhooks
- OPENAI_API_KEY env var set

### Quick Setup

```bash
# Install dependencies
npm install

# Run the final version w/ cli
npx tsx src/index.ts

# OR run the final version w/ http
npx tsx src/server.ts
```



