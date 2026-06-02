package com.example.vulkanoapp;

import static android.widget.Toast.LENGTH_LONG;

import android.os.Bundle;
import android.text.Spannable;
import android.text.Spanned;
import android.text.SpannedString;
import android.util.Log;
import android.widget.TextView;
import android.widget.Toast;

import androidx.appcompat.app.AppCompatActivity;

import com.example.vulkanoapp.jni.VulkanoLab;

public class MainActivity extends AppCompatActivity {
    private final static String SO_NAME = "vulkano_lab";

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        setContentView(R.layout.activity_main);
//        System.loadLibrary("iGraphicsCore.huawei");
        System.loadLibrary(SO_NAME);
        updateView();
    }

    void updateView() {
        TextView textView = this.findViewById(R.id.main_title);
        textView.setText(VulkanoLab.helloVulkano());

        textView = findViewById(R.id.main_content);
        textView.setText(VulkanoLab.vulkanoInfo());

        textView = findViewById(R.id.vulkano_create);
        textView.setText(VulkanoLab.createVulkanoDeviceQueue());

        textView = findViewById(R.id.vulkano_buffer_create);
        textView.setText(VulkanoLab.createVulkanoBuffer());

//        textView = findViewById(R.id.vulkano_compute);
//        textView.setText(VulkanoLab.vulkanoCompute());
    }

}