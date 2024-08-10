package com.bitshift.bike_aid;

import static androidx.core.app.ActivityCompat.startActivityForResult;

import android.app.Service;
import android.bluetooth.BluetoothAdapter;
import android.bluetooth.BluetoothDevice;
import android.bluetooth.BluetoothGatt;
import android.bluetooth.BluetoothGattCallback;
import android.bluetooth.BluetoothGattCharacteristic;
import android.bluetooth.BluetoothGattDescriptor;
import android.bluetooth.BluetoothGattService;
import android.bluetooth.BluetoothManager;
import android.bluetooth.BluetoothProfile;
import android.bluetooth.le.BluetoothLeScanner;
import android.bluetooth.le.ScanCallback;
import android.bluetooth.le.ScanResult;
import android.content.Context;
import android.content.Intent;
import android.os.Handler;
import android.os.IBinder;

import java.util.ArrayList;
import java.util.List;
import java.util.Set;
import java.util.UUID;

public class BLE {
    // ==== notes ====
    /*
    This class handles the ble adaptor and connection to the device
    There are callbacks at the bottom which handle the gatt server and notifications

    Basically Ive simplified the android api to:
    init()
    connect() - automatic
    disconnect() - automatic
    onRead()
    write()

     */

    // ==== variables ====
    private static final BLE mInstance = new BLE();
    private static final Signals signals = Signals.getInstance();
    private static final Logger log = Logger.getInstance();
    private BluetoothLeScanner mScanner;
    private BluetoothAdapter mAdapter;
    private boolean mScanning;
    private final Handler handler = new Handler();
    private static final long SCAN_PERIOD = 60000; // 60 seconds
    private final String DEVICE_NAME = "BScooter";
    BluetoothDevice mDevice;
    BluetoothGatt mGatt;
    boolean mDeviceFound = false;
    private Context mContext;
    ArrayList<BluetoothGattCharacteristic> mProcessedCharacteristics = new ArrayList<>();
    private boolean mProcessedCharacteristicsComplete = false;
    ArrayList<BluetoothGattCharacteristic> mReadCharacteristics = new ArrayList<>();
    private boolean mReadCharacteristicsComplete = false;
    final static UUID CHARACTERISTIC_UPDATE_NOTIFICATION_DESCRIPTOR_UUID = UUID.fromString("00002902-0000-1000-8000-00805f9b34fb");



    // ==== listener interface ====
    private OnEventListener mOnEventListener;
    public void setOnEventListener(OnEventListener listener) {
        mOnEventListener = listener;
    }
    public interface OnEventListener {
        void onRead(BluetoothGatt gatt, BluetoothGattCharacteristic characteristic, byte[] value, int status);
    }



    // ==== functions ====

    public BLE() {}


    public static BLE getInstance() {
        return mInstance;
    }


    public void init(Context c) {
        mContext = c;
        BluetoothManager bluetoothManager = (BluetoothManager) mContext.getSystemService(Context.BLUETOOTH_SERVICE);
        mAdapter = bluetoothManager.getAdapter();
        connect();
    }


    public boolean isEnabled() {
        return mAdapter.isEnabled();
    }


    public void connect() {
        // check if bluetooth is on
        if (!mAdapter.isEnabled()) {
            log.info("please enable adapter");
            return;
        }

        // check for bonded devices
        Set<BluetoothDevice> bondedDevices = mAdapter.getBondedDevices();
        if (bondedDevices != null) {
            for (BluetoothDevice dev : bondedDevices) {
                if (isMyDevice(dev)) {
                    connectDevice();
                    return;
                }
            }
        }

        // scan
        mScanner = mAdapter.getBluetoothLeScanner();
        if (mScanner == null) {
            log.info("no bluetooth adapter available, please enable bluetooth");
            return;
        }

        // device not found, scan
        if (!mDeviceFound)
            startScan();
    }

    public void connectDevice() {
        mDevice.createBond();
        // device connected will register the callbacks here
        // and so i have the gatt class to handle it
        mGatt = mDevice.connectGatt(mContext, true, mGattCallback);
    }


    // find if this is the wanted device, by name
    public boolean isMyDevice(BluetoothDevice dev) {
        if (dev == null || dev.getName() == null)
            return false;

        if (!dev.getName().equals(DEVICE_NAME))
            return false;

        mDevice = dev;
        mDeviceFound = true;
        log.info("device found");
        return true;
    }



    public void startScan() {
        if (!mScanning) {
            // Stops scanning after a predefined scan period.
            handler.postDelayed(new Runnable() {
                @Override
                public void run() {
                    log.info("stop scan");
                    mScanning = false;
                    mScanner.stopScan(mScanCallback);
                }
            }, SCAN_PERIOD);

            log.info("start scan");
            mScanning = true;
            mScanner.startScan(mScanCallback);
        } else {
            stopScan();
        }

    }

    public void stopScan() {
        mScanning = false;
        mScanner.stopScan(mScanCallback);
    }



    // ==== read & write functions ====

    public void onRead(BluetoothGatt gatt, BluetoothGattCharacteristic characteristic, byte[] value, int status) {
        if (value.length == 0)
            return;

        mOnEventListener.onRead(gatt, characteristic, value, status);
    }


    public void write(UUID service, UUID characteristic, byte[] value) {
        if (mGatt == null || mGatt.getService(service) == null) {
            log.info("write failed, device is disconnected");
            return;
        }
        BluetoothGattCharacteristic c = mGatt.getService(service).getCharacteristic(characteristic);
        mGatt.writeCharacteristic(c, value , BluetoothGattCharacteristic.WRITE_TYPE_DEFAULT);
    }



    // ==== callbacks ====


    // scan call back
    private final ScanCallback mScanCallback = new ScanCallback() {
        @Override
        public void onScanResult(int callbackType, ScanResult result) {
            //super.onScanResult(callbackType, result);
            BluetoothDevice dev = result.getDevice();
            if (isMyDevice(dev)) {
                connectDevice();
                stopScan();
            }
        }
    };


    // gatt callback
    private final BluetoothGattCallback mGattCallback = new BluetoothGattCallback() {

        public void onCharacteristicWrite (BluetoothGatt gatt, BluetoothGattCharacteristic characteristic, int status) {
            if (status == BluetoothGatt.GATT_FAILURE)
                log.info("write fail!");
            else
                log.info("write success");
        }

        @Override
        public void onConnectionStateChange(BluetoothGatt gatt, int status, int newState) {
            if (newState == BluetoothProfile.STATE_CONNECTED) {
                log.info("connected: " + mDevice.getName());

                /* TODO: some wired issue with doubling up
                // reset settings
                mReadCharacteristics = new ArrayList<>();;
                mReadCharacteristicsComplete = false;
                mProcessedCharacteristics = new ArrayList<>();;
                mProcessedCharacteristicsComplete = false;
                 */

                gatt.discoverServices();
            }
            if (newState == BluetoothProfile.STATE_DISCONNECTED) {
                log.info("disconnected from device");
                connect();
            }
        }

        @Override
        public void onCharacteristicChanged(BluetoothGatt gatt, BluetoothGattCharacteristic characteristic, byte[] value) {
            gatt.readCharacteristic(characteristic);
        }

        @Override
        public void onCharacteristicRead(BluetoothGatt gatt, BluetoothGattCharacteristic characteristic, byte[] value, int status) {
            if (!mReadCharacteristicsComplete)
                processNextCharacteristic(gatt);
            onRead(gatt, characteristic, value, status);
        }

        @Override
        public void onDescriptorWrite (BluetoothGatt gatt, BluetoothGattDescriptor descriptor, int status) {
            String s = Functions.string16FromUUID(descriptor.getCharacteristic().getUuid());
            //log.info("notify: " + s);
            processNextCharacteristic(gatt);
        }

        @Override
        public void onServicesDiscovered(BluetoothGatt gatt, int status) {
            processNextCharacteristic(gatt);
        }


        // this function will process the gatt characteristics
        // there is a delay with BLE so we need to wait till we receive the ok
        // then process the next in the list
        public void processNextCharacteristic(BluetoothGatt gatt) {
            // get list of all services
            List<BluetoothGattService> services = gatt.getServices();

            // 1. read characteristics
            if (!mReadCharacteristicsComplete) {
                // loop services
                for (BluetoothGattService s : services) {
                    List<BluetoothGattCharacteristic> characteristics = s.getCharacteristics();

                    // loop characteristics
                    for (BluetoothGattCharacteristic c : characteristics) {
                        // have we already been configured?
                        if (mReadCharacteristics.contains(c))
                            continue;

                        // check if we have read
                        boolean hasRead = (c.getProperties() & BluetoothGattCharacteristic.PROPERTY_READ) != 0;

                        // read
                        if (hasRead)
                            gatt.readCharacteristic(c);

                        // add to completed list
                        mReadCharacteristics.add(c);

                        // return until we are notified
                        // if not try another characteristic
                        if (hasRead)
                            return;
                    }
                }

                mReadCharacteristicsComplete = true;
            }

            // 2. configure notify characteristics
            if (!mProcessedCharacteristicsComplete) {
                // loop services
                for (BluetoothGattService s : services) {
                    List<BluetoothGattCharacteristic> characteristics = s.getCharacteristics();

                    // loop characteristics
                    for (BluetoothGattCharacteristic c : characteristics) {

                        // have we already been configured?
                        if (mProcessedCharacteristics.contains(c))
                            continue;

                        // check if we have notify
                        boolean hasNotify = (c.getProperties() & BluetoothGattCharacteristic.PROPERTY_NOTIFY) != 0;

                        // set notification
                        if (hasNotify) {
                            gatt.setCharacteristicNotification(c, true);
                            BluetoothGattDescriptor descriptor = c.getDescriptor(CHARACTERISTIC_UPDATE_NOTIFICATION_DESCRIPTOR_UUID);
                            if (descriptor != null)
                                gatt.writeDescriptor(descriptor, BluetoothGattDescriptor.ENABLE_NOTIFICATION_VALUE );
                        }

                        // add to completed list
                        mProcessedCharacteristics.add(c);

                        // has notify, so wait for response
                        // if not try another characteristic
                        if (hasNotify)
                            return;
                    }
                }

                mProcessedCharacteristicsComplete = true;
            }

            log.info("finish processing characteristics");
        }

    };

}
