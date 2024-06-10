Simple example how to interact with smart contracts on Vara Network

contracts folder:
- contract using contract state to store and modify vector of messages

backend folder:
- exppressjs application to interact with contract (update message in contract and get current state)
    - expressjs running on port 8085
    - to update message in contract use /api/data endpoint, post with json obj { content: "Some Text"}