package wen.suyufox.winterfox.service

import android.util.Log

class Service {
    fun pong(value: String): String {
        Log.i("Pong", value)
        return value
    }
}
