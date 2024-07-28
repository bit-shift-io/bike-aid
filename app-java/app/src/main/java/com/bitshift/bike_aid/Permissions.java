package com.bitshift.bike_aid;

import android.Manifest;
import android.app.Activity;
import android.content.pm.PackageManager;
import android.os.Build;

import androidx.core.app.ActivityCompat;
import androidx.core.content.ContextCompat;

public class Permissions {

    private int REQUEST_FINE_LOCATION_PERMISSION = 100;
    private int REQUEST_BLUETOOTH_SCAN_PERMISSION = 101;
    private int REQUEST_BACKGROUND_LOCATION_PERMISSION = 102;
    private int REQUEST_BLUETOOTH_CONNECT_PERMISSION= 103;

    Activity activity;


    Permissions (Activity a) {
        activity = a;
        request();
    }
    public void request() {
        int permissionsCode = 42;
        String[] permissions = {
                Manifest.permission.BLUETOOTH_SCAN,
                Manifest.permission.ACCESS_COARSE_LOCATION,
                Manifest.permission.ACCESS_FINE_LOCATION,
                Manifest.permission.BLUETOOTH_CONNECT
        };

        ActivityCompat.requestPermissions(activity, permissions, permissionsCode);

        /*
        if (!hasPermissions(this, permissions)) {
            ActivityCompat.requestPermissions(this, permissions, permissionsCode);
        }

        // Request the ACCESS_FINE_LOCATION permission at runtime
        if (CheckSelfPermission(Manifest.permission.ACCESS_FINE_LOCATION) != true)
        {
            RequestPermissions(new String[] { Manifest.permission.ACCESS_FINE_LOCATION },
                    REQUEST_FINE_LOCATION_PERMISSION);
        }

        if (CheckSelfPermission(Manifest.permission.ACCESS_BACKGROUND_LOCATION) != true)
        {
            RequestPermissions(new String[] { Manifest.permission.ACCESS_BACKGROUND_LOCATION },
                    REQUEST_BACKGROUND_LOCATION_PERMISSION);
        }

        // Request the BLUETOOTH_SCAN permission at runtime
        if (CheckSelfPermission(Manifest.permission.BLUETOOTH_SCAN) != true)
        {
            RequestPermissions(new String[] { Manifest.permission.BLUETOOTH_SCAN },
                    REQUEST_BLUETOOTH_SCAN_PERMISSION);
        }

        //Request the BLUETOOTH_CONNECT permission at runtime
        if (CheckSelfPermission(Manifest.permission.BLUETOOTH_CONNECT) != true)
        {
            RequestPermissions(new String[] { Manifest.permission.BLUETOOTH_CONNECT },
                    REQUEST_BLUETOOTH_CONNECT_PERMISSION);
        }

         */
    }

    private void RequestPermissions(String[] strings, int requestPermission) {
        ActivityCompat.requestPermissions(activity, strings, requestPermission);
    }

    private boolean CheckSelfPermission(String permission) {
        return ActivityCompat.checkSelfPermission(activity, permission) == PackageManager.PERMISSION_GRANTED;
    }


}
