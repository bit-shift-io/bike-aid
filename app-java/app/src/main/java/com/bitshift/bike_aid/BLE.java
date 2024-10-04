package com.bitshift.bike_aid;

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
import android.bluetooth.le.ScanFilter;
import android.bluetooth.le.ScanResult;
import android.bluetooth.le.ScanSettings;
import android.content.Context;
import android.os.Handler;

import java.util.ArrayList;
import java.util.Collections;
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
    ArrayList<BluetoothGattCharacteristic> mNotifyCharacteristics = new ArrayList<>();
    private boolean mNotifyCharacteristicsComplete = false;
    ArrayList<BluetoothGattCharacteristic> mReadCharacteristics = new ArrayList<>();
    private boolean mReadCharacteristicsComplete = false;
    final static UUID CHARACTERISTIC_UPDATE_NOTIFICATION_DESCRIPTOR_UUID = UUID.fromString("00002902-0000-1000-8000-00805f9b34fb");
    private BluetoothManager mBluetoothManager;



    // ==== listener interface ====
    public interface OnReadListener {
        void onRead(BluetoothGatt gatt, BluetoothGattCharacteristic characteristic, byte[] value);
    }
    private OnReadListener mOnReadListener;
    public void setOnReadListener(OnReadListener listener) { this.mOnReadListener = listener; }

    public interface OnRequestEnableBLEListener {
        void onRequestEnableBLE();
    }
    private OnRequestEnableBLEListener mOnRequestEnableBLEListener;
    public void setOnRequestEnableBLEListener(OnRequestEnableBLEListener listener) { this.mOnRequestEnableBLEListener = listener; }



    // ==== functions ====

    public BLE() {}


    public static BLE getInstance() {
        return mInstance;
    }


    public void init(Context c) {
        mContext = c;
        mBluetoothManager = (BluetoothManager) mContext.getSystemService(Context.BLUETOOTH_SERVICE);
        mAdapter = mBluetoothManager.getAdapter();
    }


    public boolean isEnabled() {
        return mAdapter.isEnabled();
    }


    public void close() {
        log.info("close");
        if (mGatt != null) {
            mGatt.disconnect();
            mGatt.close();
        }
        mGatt = null;
        mDeviceFound = false;
        mScanning = false;
        //resetSettings(true, true);
    }


    public void resetSettings(boolean readCharacteristics, boolean notifyCharacteristics) {
        if (readCharacteristics) {
            mReadCharacteristics = new ArrayList<>();
            mReadCharacteristicsComplete = false;
        }
        if (notifyCharacteristics) {
            mNotifyCharacteristics = new ArrayList<>();
            mNotifyCharacteristicsComplete = false;
        }
    }

    public void connectDelay() {
        // used when we are closing first, the need to wait for that to complete before reconnecting
        handler.postDelayed(new Runnable() {
            @Override
            public void run() {
                connect();
            }
        }, 2000);
    }


    public void connect() {
        // check if bluetooth is on
        if (!mAdapter.isEnabled()) {
            log.info("enable bluetooth");
            mOnRequestEnableBLEListener.onRequestEnableBLE();
            return;
        }

        // check for existing connection
        List<BluetoothDevice> connectedDevices = mBluetoothManager.getConnectedDevices(BluetoothProfile.GATT);
        if (connectedDevices != null) {
            for (BluetoothDevice dev : connectedDevices) {
                if (isMyDevice(dev)) {
                    log.info("existing connected device");
                    resetSettings(true, false); // reread only, i think it must reconfig notifications
                    connectDevice();
                    return;
                }
            }
        }

        // check for bonded devices
        Set<BluetoothDevice> bondedDevices = mAdapter.getBondedDevices();
        if (bondedDevices != null) {
            for (BluetoothDevice dev : bondedDevices) {
                if (isMyDevice(dev)) {
                    log.info("existing bonded device");
                    resetSettings(true, false); // reread only, i think it must reconfig notifications
                    connectDevice();
                    return;
                }
            }
        }

        // scan for new device
        mScanner = mAdapter.getBluetoothLeScanner();
        resetSettings(true, true); // reset notify & read as we have a new connection
        if (!mDeviceFound) startScan();
    }

    public void connectDevice() {
        // stop scan if scanning
        stopScan();

        // we dont need to bond for BLE
        //mDevice.createBond();

        // disable autoconnect, as my scanning is faster
        mGatt = mDevice.connectGatt(mContext, false, mGattCallback, BluetoothDevice.TRANSPORT_LE);
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
        if (mScanning) {
            stopScan();
        }

        ScanFilter filter = new ScanFilter.Builder()
                .setDeviceName("BScooter") // Filter by device name
                .build();

        // optimised for speed
        ScanSettings settings = new ScanSettings.Builder()
                .setScanMode(ScanSettings.SCAN_MODE_LOW_LATENCY)
                .setCallbackType(ScanSettings.CALLBACK_TYPE_ALL_MATCHES) // CALLBACK_TYPE_FIRST_MATCH
                .setMatchMode(ScanSettings.MATCH_MODE_AGGRESSIVE)
                .setNumOfMatches(ScanSettings.MATCH_NUM_ONE_ADVERTISEMENT)
                .setReportDelay(0L)
                .build();

        log.info("start scan");
        mScanning = true;
        mScanner.startScan(Collections.singletonList(filter), settings, mScanCallback);
    }


    public void stopScan() {
        if (!mScanning) return;

        log.info("stop scan");
        mScanning = false;
        mScanner.stopScan(mScanCallback);
    }



    // ==== read & write functions ====

    public void onRead(BluetoothGatt gatt, BluetoothGattCharacteristic characteristic, byte[] value) {
        if (value.length == 0)
            return;

        mOnReadListener.onRead(gatt, characteristic, value);
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
            }
        }

        @Override
        public void onScanFailed(int errorCode) {
            log.info("scan failed");
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
            // newState = STATE_CONNECTED, STATE_DISCONNECTED, STATE_CONNECTING, STATE_DISCONNECTING
            // status = GATT_SUCCESS, GATT_ERROR
            // bondState = BOND_NONE, BOND_BONDING or BOND_BONDED

            // if we need bonding stuff
            // https://medium.com/@martijn.van.welie/making-android-ble-work-part-2-47a3cdaade07
            //int bondState = mDevice.getBondState();

            if(status == BluetoothGatt.GATT_SUCCESS) {
                if (newState == BluetoothProfile.STATE_CONNECTED) {
                    // We successfully connected, proceed with service discovery
                    log.info("connected: " + mDevice.getName());
                    gatt.discoverServices();
                } else if (newState == BluetoothProfile.STATE_DISCONNECTED) {
                    // We successfully disconnected on our own request
                    log.info("disconnected from device");
                    close();
                    connectDelay();
                } else {
                    // We're CONNECTING or DISCONNECTING, ignore for now
                }
            } else {
                // An error happened...figure out what happened!
                log.info("error connecting to device");
                close();
                connectDelay();
            }
        }

        @Override
        public void onCharacteristicChanged(BluetoothGatt gatt, BluetoothGattCharacteristic characteristic, byte[] value) {
            // limit on size of data, so for uart tx request a manual read instead
            if (characteristic.getUuid().toString().equalsIgnoreCase("6E400003-B5A3-F393-E0A9-E50E24DCCA9E")) {
                gatt.readCharacteristic(characteristic);
                return;
            }

            // Copy the byte array so we have a threadsafe copy
            final byte[] value_copy = new byte[value.length];
            System.arraycopy(value, 0, value_copy, 0, value.length );

            // Characteristic has new value so pass it on for processing
            handler.post(new Runnable() {
                @Override
                public void run() { onRead(gatt, characteristic, value); }
            });


            // new method above seems to have a limit on the size of the uart strings....
            // https://medium.com/@martijn.van.welie/making-android-ble-work-part-3-117d3a8aee23
            //gatt.readCharacteristic(characteristic);
        }

        @Override
        public void onCharacteristicRead(BluetoothGatt gatt, BluetoothGattCharacteristic characteristic, byte[] value, int status) {
            if (!mReadCharacteristicsComplete)
                processNextCharacteristic(gatt);

            // Copy the byte array so we have a threadsafe copy
            final byte[] value_copy = new byte[value.length];
            System.arraycopy(value, 0, value_copy, 0, value.length );

            handler.post(new Runnable() {
                @Override
                public void run() { onRead(gatt, characteristic, value_copy); }
            });
        }

        @Override
        public void onDescriptorWrite (BluetoothGatt gatt, BluetoothGattDescriptor descriptor, int status) {
            String s = Functions.string16FromUUID(descriptor.getCharacteristic().getUuid());
            //log.info("notify: " + s);
            processNextCharacteristic(gatt);
        }

        @Override
        public void onServicesDiscovered(BluetoothGatt gatt, int status) {
            // Check if the service discovery succeeded. If not disconnect
            if (status != BluetoothGatt.GATT_SUCCESS) {
                log.info("service discovery failed");
                close();
                return;
            }

            // success
            log.info("begin service discovery");
            processNextCharacteristic(gatt);
        }


        // this function will process the gatt characteristics
        // there is a delay with BLE so we need to wait till we receive the ok
        // then process the next in the list
        public void processNextCharacteristic(BluetoothGatt gatt) {
            // get list of all services
            // note these are cached
            // https://medium.com/@martijn.van.welie/making-android-ble-work-part-2-47a3cdaade07
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
            if (!mNotifyCharacteristicsComplete) {
                // loop services
                for (BluetoothGattService s : services) {
                    List<BluetoothGattCharacteristic> characteristics = s.getCharacteristics();

                    // loop characteristics
                    for (BluetoothGattCharacteristic c : characteristics) {

                        // have we already been configured?
                        if (mNotifyCharacteristics.contains(c))
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
                        mNotifyCharacteristics.add(c);

                        // has notify, so wait for response
                        // if not try another characteristic
                        if (hasNotify)
                            return;
                    }
                }

                mNotifyCharacteristicsComplete = true;
            }

            log.info("complete service discovery");
            log.info("ready!");
        }

    };

}
