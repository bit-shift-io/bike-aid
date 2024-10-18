package com.bitshift.bike_aid;

import android.annotation.SuppressLint;
import android.bluetooth.BluetoothGatt;
import android.bluetooth.BluetoothGattCharacteristic;
import android.os.Handler;
import android.os.Looper;

import java.nio.charset.StandardCharsets;
import java.util.UUID;

public class Signals {

    // ==== notes ====
    /*
    This class interfaces between the gui and the data
    */


    // ==== variables ====
    private static final Logger log = Logger.getInstance();
    private final BLE ble = BLE.getInstance();
    private static final Signals mInstance = new Signals();
    public static Signals getInstance() {
        return mInstance;
    }
    public boolean alarm_on = false;
    public boolean power_on = false;


    // ==== listener interface ====
    private OnEventListener mOnEventListener;
    public void setOnEventListener(OnEventListener listener) {
        mOnEventListener = listener;
    }
    public interface OnEventListener {
        void onTemperature(String result);
        void onSpeed(String result);
        void onClockMinutes(String result);
        void onClockHours(String result);
        void onCruiseLevel(int result);
        void onPower(boolean result);
        void onAlarm(boolean result);
        void onBatteryLevel(String result);
        void onBrake(boolean result);
        void onParkBrake(boolean result);
        void onBatteryPower(String result);
    }


    // ==== functions ====
    private Signals () {
        ble.setOnReadListener(this::onRead);
    }


    // ==== gui -> data ====
    public void togglePower() {
        UUID service_id = Functions.uuidFrom16("1000");
        UUID characteristic_id = Functions.uuidFrom16("1001");
        byte[] b;
        if (power_on)
            b = new byte[]{ (byte) 0 };
        else
            b = new byte[]{ (byte) 183 };

        ble.write(service_id, characteristic_id, b);
    }

    public void toggleAlarm() {
        UUID service_id = Functions.uuidFrom16("1000");
        UUID characteristic_id = Functions.uuidFrom16("1004");
        byte[] b;
        if (alarm_on)
            b = new byte[]{ (byte) 0 };
        else
            b = new byte[]{ (byte) 205 };

        ble.write(service_id, characteristic_id, b);
    }

    public void setUART(String s) {
        log.info("> " + s);
        UUID uart_service = UUID.fromString("6E400001-B5A3-F393-E0A9-E50E24DCCA9E");
        UUID uart_write_characteristic = UUID.fromString("6E400002-B5A3-F393-E0A9-E50E24DCCA9E");
        byte[] b = s.getBytes(StandardCharsets.UTF_8);
        ble.write(uart_service, uart_write_characteristic, b);
    }


    // ==== on ble read ====
    @SuppressLint("DefaultLocale")
    public void onRead(BluetoothGatt gatt, BluetoothGattCharacteristic characteristic, byte[] value) {
        String id = Functions.string16FromUUID(characteristic.getUuid());


        // debug read
        //String st = new String(value, StandardCharsets.UTF_8);
        //log.info("read " + id + " " + st + " " + value.length);

        // uart

        // rx - 6E400002-B5A3-F393-E0A9-E50E24DCCA9E
        // is never read, write only

        // tx - 6E400003-B5A3-F393-E0A9-E50E24DCCA9E
        if (id.equals("0003"))
            log.info(new String(value, StandardCharsets.UTF_8));


        // 1000 series is settings

        // power switch
        if (id.equals("1001"))
            new Handler(Looper.getMainLooper()).post(() -> mOnEventListener.onPower(value[0] != 0));

        // alarm switch
        if (id.equals("1004"))
            new Handler(Looper.getMainLooper()).post(() -> mOnEventListener.onAlarm(value[0] != 0));



        // 2000 series is data

        // speed
        if (id.equals("2001"))
            new Handler(Looper.getMainLooper()).post(() -> mOnEventListener.onSpeed(String.format("%02d", value[0])));

        // temperature
        if (id.equals("2004"))
            new Handler(Looper.getMainLooper()).post(() -> mOnEventListener.onTemperature(String.format("%02d", value[0])));

        // clock minutes
        if (id.equals("2005"))
            new Handler(Looper.getMainLooper()).post(() -> mOnEventListener.onClockMinutes(String.format("%02d", value[0])));

        // clock hours
        if (id.equals("2006"))
            new Handler(Looper.getMainLooper()).post(() -> mOnEventListener.onClockHours(String.format("%02d", value[0])));

        // brake
        if (id.equals("2007"))
            new Handler(Looper.getMainLooper()).post(() -> mOnEventListener.onBrake(value[0] != 0));

        // park brake
        if (id.equals("2008"))
            new Handler(Looper.getMainLooper()).post(() -> mOnEventListener.onParkBrake(value[0] != 0));

        // cruise
        if (id.equals("2009"))
            new Handler(Looper.getMainLooper()).post(() -> mOnEventListener.onCruiseLevel(value[0]));


        // 0x180f series is battery

        // level - 0x2a19
        if (id.equals("2a19"))
            new Handler(Looper.getMainLooper()).post(() -> mOnEventListener.onBatteryLevel(String.valueOf(value[0])));

        // power
        if (id.equals("2b05")) {
            int v = (value[0] & 0xFF) | ((value[1] & 0xFF) << 8); // 16 bit value
            new Handler(Looper.getMainLooper()).post(() -> mOnEventListener.onBatteryPower(String.valueOf(v)));
        }

    }
}
