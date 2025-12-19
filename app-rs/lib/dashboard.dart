import 'package:flutter/material.dart';
import 'package:bike_aid/src/rust/api/protocol.dart';

class ScooterDashboard extends StatelessWidget {
  final ScooterState? state;
  final bool isConnecting;
  final bool isScanning;
  final Function(ScooterCommand) onSendCommand;

  const ScooterDashboard({
    super.key,
    required this.state,
    required this.isConnecting,
    required this.isScanning,
    required this.onSendCommand,
  });

  @override
  Widget build(BuildContext context) {
    return Container(
      height: 480, // Fixed height for dashboard density
      color: const Color(0xFF1A1A1A),
      child: Column(
        children: [
          _buildRow1Buttons(),
          const Divider(height: 1, color: Colors.white24, thickness: 1),
          _buildRow2Stats(),
          const Divider(height: 1, color: Colors.white24, thickness: 1),
          Expanded(child: _buildRow3MainContent()),
          const Divider(height: 1, color: Colors.white24, thickness: 1),
          _buildRow4BottomStats(),
          const Divider(height: 1, color: Colors.white24, thickness: 1),
          _buildRow5Buttons(),
        ],
      ),
    );
  }

  Widget _buildRow1Buttons() {
    return SizedBox(
      height: 90,
      child: Row(
        children: [
          Expanded(child: _buildDashboardButton("Power", state?.powerOn ?? false, () => onSendCommand(const ScooterCommand.togglePower()))),
          const VerticalDivider(width: 1, color: Colors.white24, thickness: 1),
          Expanded(child: _buildDashboardButton("Alarm", state?.alarmOn ?? false, () => onSendCommand(const ScooterCommand.toggleAlarm()))),
          const VerticalDivider(width: 1, color: Colors.white24, thickness: 1),
          Expanded(child: _buildDashboardButton("Lights", state?.lightsOn ?? false, () => onSendCommand(const ScooterCommand.toggleLights()))),
        ],
      ),
    );
  }

  Widget _buildRow2Stats() {
    return Padding(
      padding: const EdgeInsets.symmetric(vertical: 4, horizontal: 10),
      child: Row(
        mainAxisAlignment: MainAxisAlignment.spaceBetween,
        children: [
          _buildStatText("${state?.batteryPower ?? "0000"} w", fontSize: 26),
          _buildClockWidget(),
          _buildStatText("${state?.batteryLevel ?? "0"}%", fontSize: 26),
        ],
      ),
    );
  }

  Widget _buildRow3MainContent() {
    return Padding(
      padding: const EdgeInsets.symmetric(horizontal: 10),
      child: Row(
        children: [
          // Left: Indicators
          SizedBox(
            width: 100,
            child: Column(
              mainAxisAlignment: MainAxisAlignment.spaceEvenly,
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                _buildIndicatorLabel("Brake", state?.brakeActive ?? false),
                _buildIndicatorLabel("Cruise", (state?.cruiseLevel ?? 0) > 0),
                _buildIndicatorLabel("Park", state?.parkBrakeActive ?? false),
              ],
            ),
          ),
          
          // Center: Speed
          Expanded(
            child: Column(
              mainAxisAlignment: MainAxisAlignment.center,
              children: [
                Row(
                  mainAxisAlignment: MainAxisAlignment.center,
                  crossAxisAlignment: CrossAxisAlignment.baseline,
                  textBaseline: TextBaseline.alphabetic,
                  children: [
                    Text(
                      state?.speed ?? "00",
                      style: const TextStyle(fontSize: 100, fontWeight: FontWeight.normal, color: Color(0xFFCCCCCC), height: 1.0),
                    ),
                    const Text(" km/h", style: TextStyle(fontSize: 24, color: Color(0xFFCCCCCC))),
                  ],
                ),
                Text(
                  "${state?.throttleLevel ?? "0000"} mv",
                  style: const TextStyle(fontSize: 24, color: Color(0xFFCCCCCC)),
                ),
              ],
            ),
          ),

          // Right: Cruise Bar Chart (5 bars)
          SizedBox(
            width: 80,
            child: Column(
              mainAxisAlignment: MainAxisAlignment.center,
              children: List.generate(5, (index) {
                final level = 5 - index;
                final isActive = (state?.cruiseLevel ?? 0) >= level;
                return Align(
                  alignment: Alignment.centerRight,
                  child: Container(
                    margin: const EdgeInsets.symmetric(vertical: 2),
                    height: 16,
                    width: 20.0 + (level * 15.0),
                    decoration: BoxDecoration(
                      color: isActive ? const Color(0xFFBBBBBB) : const Color(0xFF333333),
                      borderRadius: BorderRadius.circular(2),
                    ),
                  ),
                );
              }),
            ),
          ),
        ],
      ),
    );
  }

  Widget _buildRow4BottomStats() {
    return Padding(
      padding: const EdgeInsets.symmetric(vertical: 8, horizontal: 10),
      child: Row(
        mainAxisAlignment: MainAxisAlignment.spaceBetween,
        children: [
          _buildStatText("${state?.odometer ?? "0000"} km", fontSize: 26),
          _buildStatText("${state?.temperature ?? "00"}°C", fontSize: 26),
          _buildStatText("${state?.clockHours ?? "00"}:${state?.clockMinutes ?? "00"}", fontSize: 26),
        ],
      ),
    );
  }

  Widget _buildRow5Buttons() {
    return SizedBox(
      height: 90,
      child: Row(
        children: [
          Expanded(child: _buildDashboardButton("Sport", state?.sportOn ?? false, () => onSendCommand(const ScooterCommand.toggleSport()))),
          const VerticalDivider(width: 1, color: Colors.white24, thickness: 1),
          Expanded(child: _buildDashboardButton("Cruise\nDown", false, () => onSendCommand(const ScooterCommand.cruiseDown()), isAction: true)),
          const VerticalDivider(width: 1, color: Colors.white24, thickness: 1),
          Expanded(child: _buildDashboardButton("Cruise\nUp", false, () => onSendCommand(const ScooterCommand.cruiseUp()), isAction: true)),
          const VerticalDivider(width: 1, color: Colors.white24, thickness: 1),
          Expanded(child: _buildDashboardButton("Horn", false, () => onSendCommand(const ScooterCommand.horn()), isAction: true)),
        ],
      ),
    );
  }

  Widget _buildDashboardButton(String label, bool isOn, VoidCallback onTap, {bool isAction = false}) {
    return Material(
      color: Colors.transparent,
      child: InkWell(
        onTap: onTap,
        child: Container(
          color: (!isAction && isOn) ? const Color(0xFF333333) : Colors.transparent,
          alignment: Alignment.center,
          child: Column(
            mainAxisAlignment: MainAxisAlignment.center,
            children: [
              Text(
                label,
                style: const TextStyle(color: Colors.white, fontSize: 20, fontWeight: FontWeight.normal),
                textAlign: TextAlign.center,
              ),
              if (!isAction)
                Text(
                  isOn ? "ON" : "OFF",
                  style: const TextStyle(color: Colors.white, fontSize: 20),
                ),
            ],
          ),
        ),
      ),
    );
  }

  Widget _buildStatText(String text, {double fontSize = 26}) {
    return Text(
      text,
      style: TextStyle(fontSize: fontSize, color: const Color(0xFFCCCCCC), fontWeight: FontWeight.normal),
    );
  }

  Widget _buildIndicatorLabel(String label, bool isActive) {
    return Text(
      label,
      style: TextStyle(
        fontSize: 26, 
        color: isActive ? Colors.white : const Color(0xFF333333),
        fontWeight: FontWeight.normal
      ),
    );
  }

  Widget _buildClockWidget() {
    return StreamBuilder(
      stream: Stream.periodic(const Duration(seconds: 1)),
      builder: (context, snapshot) {
        final now = DateTime.now();
        final hours = now.hour > 12 ? now.hour - 12 : (now.hour == 0 ? 12 : now.hour);
        final ampm = now.hour >= 12 ? "pm" : "am";
        final timeStr = "$hours:${now.minute.toString().padLeft(2, '0')}";
        return Row(
          crossAxisAlignment: CrossAxisAlignment.baseline,
          textBaseline: TextBaseline.alphabetic,
          children: [
            Text(timeStr, style: const TextStyle(fontSize: 26, color: Color(0xFFCCCCCC))),
            Text(" $ampm", style: const TextStyle(fontSize: 20, color: Color(0xFFCCCCCC))),
          ],
        );
      },
    );
  }
}
