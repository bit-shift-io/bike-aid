package com.bitshift.bike_aid;


import android.content.Context;
import android.os.Bundle;
import android.os.Handler;
import android.view.View;
import android.view.Window;
import android.widget.ScrollView;
import android.widget.TextView;
import androidx.appcompat.app.AppCompatActivity;


public class MainActivity extends AppCompatActivity {

    // variables
    private static final Logger log = Logger.getInstance();
    ScrollView logScroll;
    TextView logText;

    // functions

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        // hide action bar
        //requestWindowFeature(Window.FEATURE_NO_TITLE);
        //getSupportActionBar().hide();

        // gui
        setContentView(R.layout.activity_main);
        Context context = getApplicationContext();

        // store views
        logScroll = findViewById(R.id.scrollViewLog);
        logText = findViewById(R.id.textViewLog);

        // callaback for log updates
        log.setOnEventListener(new Logger.OnEventListener() {
            @Override
            public void onUpdate(String result) {
                logText.setText(result);
                new Handler().postDelayed(() -> logScroll.fullScroll(View.FOCUS_DOWN), 10);
            }
        });

        log.info("MainActivity.onCreate");

        // check permissions
        new Permissions(this);

        // start ble
        new BLE(context);
    }


}