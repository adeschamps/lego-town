#ifndef ARDUINO_HPP
#define ARDUINO_HPP

// This class encapsulates the basic logic of an Arduino program.
// It differs from an actual Arduino in that there is no setup(),
// because constructors serve the same role, and loop() returns a
// value which can be used to end execution.
class Arduino
{
public:
  // Drive the main logic of the Arduino
  void run()
  {
    while (loop() == CONTINUE);
  }

protected:
  enum LoopStatus { CONTINUE, DONE };

  virtual LoopStatus loop() = 0;
};

#endif
