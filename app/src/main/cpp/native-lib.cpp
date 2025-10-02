#include <jni.h>
#include <string>
#include <android/log.h>

// Include the curl header. CMake will find it automatically.
#include <curl/curl.h>

// A callback function for libcurl to write received data into a string.
static size_t WriteCallback(void *contents, size_t size, size_t nmemb, std::string *s) {
    size_t newLength = size * nmemb;
    try {
        s->append((char*)contents, newLength);
    } catch(std::bad_alloc &e) {
        // Handle memory allocation error
        return 0;
    }
    return newLength;
}

extern "C" JNICALL
Java_com_example_nativeapp_MainActivity_stringFromJNI(
        JNIEnv* env,
        jobject /* this */) {

    // Initialize CURL
    CURL *curl;
    CURLcode res;
    std::string readBuffer; // This will store the response from the server

    curl_global_init(CURL_GLOBAL_DEFAULT);
    curl = curl_easy_init();

    if (curl) {
        // Set the URL for the request (using an example API)
        curl_easy_setopt(curl, CURLOPT_URL, "https://jsonplaceholder.typicode.com/todos/1");

        // Set the callback function to handle the response data
        curl_easy_setopt(curl, CURLOPT_WRITEFUNCTION, WriteCallback);
        curl_easy_setopt(curl, CURLOPT_WRITEDATA, &readBuffer);

        // For HTTPS, libcurl needs to verify the peer's certificate.
        // On Android, the CA bundle path is not standard. For simplicity here,
        // we can disable peer verification, but for production, you should set
        // CURLOPT_CAINFO to a valid CA bundle.
        // More info: https://curl.se/libcurl/c/CURLOPT_SSL_VERIFYPEER.html
        curl_easy_setopt(curl, CURLOPT_SSL_VERIFYPEER, 0L);

        // Perform the request
        res = curl_easy_perform(curl);

        // Check for errors
        if (res != CURLE_OK) {
            std::string error_msg = "curl_easy_perform() failed: ";
            error_msg += curl_easy_strerror(res);
            __android_log_write(ANDROID_LOG_ERROR, "NativeApp", error_msg.c_str());
            readBuffer = error_msg; // Return error to UI for debugging
        } else {
            __android_log_print(ANDROID_LOG_INFO, "NativeApp", "Curl request successful. Response size: %zu", readBuffer.length());
        }

        // Cleanup
        curl_easy_cleanup(curl);
    }

    curl_global_cleanup();

    // Return the response body (or error) as a JNI string
    return env->NewStringUTF(readBuffer.c_str());
}

// Make sure your MainActivity.java has a matching package name, e.g., com.example.nativeapp
