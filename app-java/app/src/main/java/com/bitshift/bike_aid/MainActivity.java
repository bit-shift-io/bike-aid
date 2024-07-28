package com.bitshift.bike_aid;

import android.bluetooth.BluetoothManager;
import android.content.Context;
import android.os.Bundle;
import android.widget.ScrollView;
import android.widget.TextView;

import androidx.appcompat.app.AppCompatActivity;

public class MainActivity extends AppCompatActivity {

    // variables

    Logger log;

    // functions

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        setContentView(R.layout.activity_main);
        Context context = getApplicationContext();

        // init log
        ScrollView s = findViewById(R.id.scrollViewLog);
        TextView t = findViewById(R.id.textViewLog);
        log = new Logger(t, s);

        log.info("MainActivity.onCreate");

        // check permissions
        new Permissions(this);

        // start ble
        new BLE(context, log);
    }


}