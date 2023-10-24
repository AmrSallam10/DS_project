# Project Requirements

- [] Multiple servers and multiple clients
- [] P2P communication between servers and also between clients
- [] A client multicasts requests to the servers, but receives a unicast reply
- [] Servres, all together, implement a distributed election algorithm to select the one to carry the incoming request (`bully algorithm`)
- [] The same algorithm is to be extended for simulating failures
- 

## Bully Algorithm

### Assumptions

1. Each process has a unique priority number.
2. All processes in the system are fully connected.
3. The process with the highest priority number will be elected as coordinator.
4. Each process knows the process number of all other processes.
5. What the process doesn’t know is which process is up or down.
6. During recovery, the failed process can take appropriate steps to resume the set of active processes.

### Messages

There can be three types of messages that processes exchange with each other:

1. **Election message**: Sent to announce election.
2. **OK message**: Responds to the Election message.
3. **Coordinator message**: Sent by winner of the election to announce the new coordinator.

### Steps

1. A node detects the failur of the coordinator node
2. It sends an election message to all processes with Process ID greater than its and awaits a response from the processes.
3. If no one responds, it wins the election and become the coordinator.
4. If any of the processes with higher Process ID responds with OK, the node with the highest ID will be announced as the coordinator.
5. The chosen node sends a coordinator message to the other nodes
6. If a previously down node comes back, it can initiate the election process,

<div style="text-align:center">

![Algorithm illustration](https://media.geeksforgeeks.org/wp-content/uploads/20230921011634/WhatsApp-Image-2023-09-21-at-011208.jpg)

</div>

