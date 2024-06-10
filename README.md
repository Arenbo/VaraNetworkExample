Simple example how to interact with smart contracts on Vara Network

contract-string folder:
- contract using String to store and modify message, state will return current message

contract-vec folder:
- contract using Vec to store all incoming messages, state will return all messages sent to contract

backend folder:
- exppressjs application to interact with contract (update message in contract and get current state)
    - expressjs running on port 8085
    - to update message in contract use /api/data endpoint, post with json payload { content: "Some Text"}