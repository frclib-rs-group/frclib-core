# Understanding

Because the api has to use C abi the Notifiers are specified with an iteger handle that correlates to a struct
in the HandleResourceContainer.

## Upon first notifier initialization

- An atexit handler is registered to free all the resources
- Notifier thread is spawned and priority is set
- Creates the FPGAInterrupt alarm
- Creates the Notifier struct and returns its handle

## Notifier thread

Waits for the FPGAInterrupt alarm to trigger and then calls the callback function.
The callback function for any notifier thats trigger time is less than the current time,
wakes to condvar, sets the triggered time to FPGATime. The callback also reprimes the alarm
for the next closest notifier.

## Notifier UpdateAlarm

Gets the Notifier from Handle, sets the trigger time and "removes" triggered time.
If the new trigger time is less than the current trigger time, the alarm is reprimed with the new trigger time.
Enables alarm if it is disabled.

## Notifier WaitForAlarm

Gets the Notifier from Handle, gets a unique lock on the mutex, and waits for the condvar to be notified.
Automatically wakes convar if triggered time is set or the notifier is disabled.
