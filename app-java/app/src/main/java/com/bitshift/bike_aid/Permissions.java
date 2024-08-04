package com.bitshift.bike_aid;

import android.Manifest;
import android.app.Activity;
import android.content.pm.PackageManager;
import android.os.Build;

import androidx.core.app.ActivityCompat;
import androidx.core.content.ContextCompat;

public class Permissions {
    // ==== notes ====
    /*
    This class handles the android permissions
     */

    // ==== functions ====
    Permissions (Activity a) {
        int permissionsCode = 42;
        String[] permissions = {
                Manifest.permission.BLUETOOTH_SCAN,
                Manifest.permission.ACCESS_COARSE_LOCATION,
                Manifest.permission.ACCESS_FINE_LOCATION,
                Manifest.permission.BLUETOOTH_CONNECT
        };

        ActivityCompat.requestPermissions(a, permissions, permissionsCode);
    }
}
