package com.bassmd.nativeapp

import androidx.appcompat.app.AppCompatActivity
import android.os.Bundle
import android.widget.TextView
import com.bassmd.nativeapp.databinding.ActivityMainBinding

class MainActivity : AppCompatActivity() {

    private lateinit var binding: ActivityMainBinding

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

        binding = ActivityMainBinding.inflate(layoutInflater)
        setContentView(binding.root)

        // Example of a call to a native method
        binding.sampleText.text = stringFromJNIC()
        binding.sampleText2.text = stringFromJNI()
    }

    /**
     * A native method that is implemented by the 'nativeapp' native library,
     * which is packaged with this application.
     */
    external fun stringFromJNI(): String

    external fun stringFromJNIC(): String

    companion object {
        init {
            System.loadLibrary("rust")
            System.loadLibrary("nativeapp")
        }
    }
}