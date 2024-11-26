# List of States with Explanations

| **State**               | **Description**                                                                 | **Key Actions**                                                                                   |
|--------------------------|---------------------------------------------------------------------------------|----------------------------------------------------------------------------------------------------|
| **Idle**                | The EVSE is not engaged in any active session. Available for new connections.   | Monitor idle energy consumption. Transition to `Reserved` or `Waiting for Authorization`.         |
| **Reserved**            | The EVSE is reserved for a specific user or session.                           | Accept remote start commands. Reset to `Idle` if the reservation expires.                         |
| **Waiting for Authorization** | The EV is connected, waiting for user or vehicle authorization.                | Validate user credentials or Plug-and-Charge certificates. Transition to `Preparing to Charge`.    |
| **Preparing to Charge**  | Authorization is complete; EVSE is preparing to deliver power.                 | Send tariff and pricing details. Handle grid constraints. Transition to `Charging`.               |
| **Charging**            | Energy transfer to the EV is active.                                           | Send periodic `MeterValues`. Handle pause commands or disconnection. Transition to `Discharging`. |
| **Discharging**         | Energy is discharged from the EV to the grid (V2G functionality).              | Monitor energy returned to the grid. Transition to `Idle` when discharging is complete.           |
| **Limited Charging**    | Charging is reduced due to grid constraints or demand response requirements.   | Update `ChargingProfile`. Transition back to `Charging` when constraints are removed.             |
| **Paused**              | Charging or discharging is temporarily stopped.                                | Resume charging or discharging. Transition to `Idle` if session times out.                        |
| **Overstay**            | EV remains connected after charging is complete.                               | Notify user. Apply overstay charges. Transition to `Idle` when unplugged.                         |
| **Battery Maintenance** | Charging is adjusted to maintain the EV battery’s health. The `TransactionEvent` message can be used to notify the CSMS about a state transition or event related to battery maintenance. The `TriggerReason` field can include a reason such as `Other` (if no predefined reason fits) or use a custom extension. | Notify user and CSMS. Transition back to `Charging` when maintenance is complete.                 |
| **Faulted**             | Hardware or software error has occurred.                                       | Notify CSMS. Attempt recovery. Transition to `Idle` if resolved.                                  |
| **Unavailable**         | EVSE is set to an unavailable state or persistent faults exist.                | Monitor for recovery or manual reset. Transition to `Idle` when resolved.                         |




# State Transition Table

| **State**                | **Trigger/Event**                                      | **Action**                                                                                          | **Next State**              |
|---------------------------|-------------------------------------------------------|-----------------------------------------------------------------------------------------------------|-----------------------------|
| **Idle**                 | EVSE is powered on                                    | EVSE sends `StatusNotification` with `Available`                                                   | **Idle**                   |
| **Idle**                 | Reservation request received                          | Reserve the connector, send `TransactionEvent` (Reserved)                                           | **Reserved**               |
| **Idle**                 | EV plugged in                                         | Send `TransactionEvent` (Started) with `TriggerReason = CablePluggedIn`                             | **Waiting for Authorization** |
| **Reserved**             | Remote start command received                         | Send `TransactionEvent` (Started) with `TriggerReason = RemoteStart`                                | **Waiting for Authorization** |
| **Reserved**             | Reservation expires                                   | Send `TransactionEvent` (Ended) with `TriggerReason = ReservationExpired`                          | **Idle**                   |
| **Waiting for Authorization** | User authorizes or Plug-and-Charge validated         | Send `Authorize` or validate Plug-and-Charge certificate                                            | **Preparing to Charge**    |
| **Waiting for Authorization** | Authorization timeout or invalid credentials         | Send `TransactionEvent` (Ended) with `TriggerReason = Timeout`                                      | **Idle**                   |
| **Preparing to Charge**   | Authorization accepted                               | Send `TransactionEvent` (Updated), send `CostUpdated` with tariff and pricing                       | **Charging**               |
| **Preparing to Charge**   | Grid constraint detected                              | Apply charging limits, update `ChargingProfile`, notify CSMS                                        | **Limited Charging**       |
| **Charging**              | Energy transfer begins                               | Periodically send `MeterValues` and cost updates (`CostUpdated`)                                    | **Charging**               |
| **Charging**              | Discharge request from CSMS or EV                    | Switch to discharging mode, send `TransactionEvent` (Updated)                                       | **Discharging**            |
| **Charging**              | User pauses session                                  | Send `TransactionEvent` (Updated) with `TriggerReason = Paused`, notify user                        | **Paused**                 |
| **Charging**              | Overstay detected (EV remains after session ends)    | Notify user, send `CostUpdated` with overstay charges                                               | **Overstay**               |
| **Charging**              | Battery health management triggers trickle charging  | Apply reduced charging rate, notify user and CSMS                                                  | **Battery Maintenance**    |
| **Discharging**           | Discharge completes or EV unplugged                  | Send `TransactionEvent` (Ended) with final energy and pricing data                                  | **Idle**                   |
| **Limited Charging**      | Grid constraints removed                             | Restore full power delivery, send `TransactionEvent` (Updated)                                      | **Charging**               |
| **Overstay**              | EV unplugged                                         | Reset EVSE status, send `TransactionEvent` (Ended)                                                 | **Idle**                   |
| **Overstay**              | User clears overstay charges                         | Notify CSMS of payment completion, reset EVSE                                                      | **Idle**                   |
| **Battery Maintenance**   | Maintenance complete or session timeout              | Transition back to normal charging, notify CSMS                                                    | **Charging**               |
| **Paused**                | Resume command received                              | Send `TransactionEvent` (Updated) with `TriggerReason = Resume`                                     | **Charging**               |
| **Paused**                | Timeout or EV unplugged                              | Send `TransactionEvent` (Ended)                                                                    | **Idle**                   |
| **Idle**                 | Regular idle energy consumption monitoring            | Send `MeterValues` for idle energy usage, notify CSMS if thresholds exceeded                        | **Idle**                   |
| **Faulted**               | Hardware fault detected                              | Send `StatusNotification` with `Faulted`, attempt recovery                                          | **Idle** or **Unavailable**|
| **Unavailable**          | EVSE set to `Available` by CSMS                      | Send `StatusNotification` with `Available`                                                         | **Idle**                   |

