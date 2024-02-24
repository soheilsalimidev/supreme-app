package com.app.webapp.utility

import android.annotation.SuppressLint
import android.app.DownloadManager
import android.content.BroadcastReceiver
import android.content.Context
import android.content.Context.DOWNLOAD_SERVICE
import android.content.Intent
import android.content.IntentFilter
import android.graphics.Bitmap
import android.net.Uri
import android.os.Build
import android.os.Environment
import android.text.TextUtils
import android.util.Log
import android.view.View.GONE
import android.view.View.VISIBLE
import android.webkit.*
import android.widget.Toast
import androidx.annotation.RequiresApi
import androidx.appcompat.app.AppCompatActivity
import com.app.webapp.Config
import com.app.webapp.R
import com.app.webapp.activities.binding
//import com.google.android.material.snackbar.Snackbar
import java.io.UnsupportedEncodingException
import java.net.URLDecoder


class MyWebViewClient(private val context: AppCompatActivity) : WebViewClient() {

    var downloadID: Long = 0
    var noInternet = false

    init {
        handleDownload()
    }

    override fun onPageStarted(view: WebView?, url: String?, favicon: Bitmap?) {
        super.onPageStarted(view, url, favicon)
        binding.webView.visibility = VISIBLE
        binding.noInternet.visibility = GONE
        binding.swipeRefreshLayout.isRefreshing = false
        binding.loading.visibility = GONE
        binding.loading.cancelAnimation()
    }

    override fun onPageCommitVisible(view: WebView?, url: String?) {
        super.onPageCommitVisible(view, url)
        binding.loading.visibility = GONE
        binding.loading.cancelAnimation()
        binding.swipeRefreshLayout.isRefreshing = false
        if (!noInternet) binding.noInternet.visibility = GONE
    }

    override fun onPageFinished(view: WebView?, url: String?) {
        super.onPageFinished(view, url)
    }

    @RequiresApi(Build.VERSION_CODES.M)
    override fun onReceivedError(view: WebView?, request: WebResourceRequest?, error: WebResourceError?) {
        super.onReceivedError(view, request, error)
        Log.i("TAG", "onReceivedError: " + error?.description.toString())
        binding.loading.visibility = GONE
        binding.loading.cancelAnimation()
        binding.noInternet.visibility = VISIBLE
        noInternet = true
        binding.webView.visibility = GONE
        if (Config(context).configType.noInternetLayout.type == 1) binding.noInternetLottieView.setImageResource(R.drawable.no_internet) else binding.noInternetLottieView.playAnimation()
        binding.tryAgain.setOnClickListener {
            binding.loading.visibility = VISIBLE
            binding.loading.playAnimation()
            binding.noInternet.visibility = GONE
            binding.noInternetLottieView.cancelAnimation()
            binding.webView.reload()
        }
        binding.swipeRefreshLayout.isRefreshing = false
    }

    override fun shouldOverrideUrlLoading(view: WebView?, request: WebResourceRequest?): Boolean {

        val url: String = request?.url.toString()

        if (url.contains("youtube.com") || url.contains("play.google.com") || url.contains("google.com/maps") || url.contains("facebook.com") || url.contains("twitter.com") || url.contains("instagram.com")) {
            context.startActivity(Intent(Intent.ACTION_VIEW, Uri.parse(url)))
        } else if (url.startsWith("mailto")) handleMailToLink(url)
        else if (url.startsWith("tel:")) context.startActivity(Intent(Intent.ACTION_DIAL, Uri.parse(url)))
        else if (url.startsWith("sms:")) handleSMSLink(url)
        else if (url.contains("geo:")) {
            val gmmIntentUri: Uri = Uri.parse(url)
            val mapIntent = Intent(Intent.ACTION_VIEW, gmmIntentUri)
            mapIntent.setPackage("com.google.android.apps.maps")
            if (mapIntent.resolveActivity(context.packageManager) != null) context.startActivity(mapIntent)

        }


        view?.loadUrl(url)
        return true
    }

    @SuppressLint("IntentReset")
    private fun handleMailToLink(url: String) { // Initialize a new intent which action is send
        val intent = Intent(Intent.ACTION_SENDTO)
        intent.type = "text/plain" // For only email app handle this intent
        intent.data = Uri.parse("mailto:")
        var mString = "" //mailto:abc@example.com?subject=Feedback&body=Message // Extract the email address from mailto url
        val to = url.substring(url.indexOf("mailto:") + 7, url.indexOf("?"))
        if (!TextUtils.isEmpty(to)) {
            val toArray = to.split(";").toTypedArray() // Put the primary email addresses array into intent
            intent.putExtra(Intent.EXTRA_EMAIL, toArray)
            mString += "TO : $to"
        }

        // Extract the subject
        if (url.contains("subject=")) {
            var subject = url.substring(url.indexOf("subject=") + 8, url.indexOf("&body"))
            Log.i("TAG", "handleMailToLink: $subject")
            if (!TextUtils.isEmpty(subject)) {
                try {
                    subject = URLDecoder.decode(subject, "UTF-8")
                } catch (e: UnsupportedEncodingException) {
                    e.printStackTrace()
                } // Put the mail subject into intent
                intent.putExtra(Intent.EXTRA_SUBJECT, subject)
                mString += "\nSUBJECT : $subject"
            }
        } else {
            mString += """
            
            No SUBJECT
            """.trimIndent()
            Log.i("TAG", "handleMailToLink: noting")
        }

        // Extract the body
        if (url.contains("body=")) {
            var body = url.substring(url.indexOf("&body=") + 6)
            if (!TextUtils.isEmpty(body)) { // Encode the body text
                try {
                    body = URLDecoder.decode(body, "UTF-8")
                } catch (e: UnsupportedEncodingException) {
                    e.printStackTrace()
                } // Put the mail body into intent
                intent.putExtra(Intent.EXTRA_TEXT, body)
                mString += "\nBODY : $body"
            }
        } else {
            mString += """
            
            No BODY
            """.trimIndent()
            Log.i("TAG", "handleMailToLink: noting")

        }
        Log.i("TAG", "handleMailToLink: $mString") // Email address not null or empty
        if (!TextUtils.isEmpty(to)) {
            if (intent.resolveActivity(context.packageManager) != null) // Finally, open the mail client activity
                context.startActivity(intent)
            else {
//                Snackbar.make(binding.root, R.string.no_email_client_found, Snackbar.LENGTH_SHORT)
//                    .show()
            }

        }
    }

    private fun handleSMSLink(url: String) {/*
            If you want to ensure that your intent is handled only by a text messaging app (and not
            other email or social apps), then use the ACTION_SENDTO action
            and include the "smsto:" data scheme
        */

        // Initialize a new intent to send sms message
        lateinit var intent: Intent

        // Extract the phoneNumber from sms url
        val phoneNumber: String = url.substring(url.indexOf(":") + 2, url.indexOf("&body"))
        intent = if (!TextUtils.isEmpty(phoneNumber)) { // Set intent data
            // This ensures only SMS apps respond
            Intent(Intent.ACTION_SENDTO, Uri.parse("smsto:$phoneNumber"))
        } else { // If the sms link built without phone number
            Intent(Intent.ACTION_SENDTO, Uri.parse("smsto:"))
        }

        // Extract the sms body from sms url
        if (url.contains("body=")) {
            var smsBody: String? = url.substring(url.indexOf("&body") + 6)

            // Encode the sms body
            try {
                smsBody = URLDecoder.decode(smsBody, "UTF-8")
            } catch (e: UnsupportedEncodingException) {
                e.printStackTrace()
            }
            if (!TextUtils.isEmpty(smsBody)) intent.putExtra("sms_body", smsBody)
            Log.i("TAG", "handleSMSLink: $smsBody")
        }

        if (intent.resolveActivity(context.packageManager) != null) context.startActivity(intent)
        else {
//            Snackbar.make(binding.root, R.string.no_sms_app, Snackbar.LENGTH_SHORT).show()
        }

    }

    @SuppressLint("UnspecifiedRegisterReceiverFlag")
    private fun handleDownload() {
        binding.webView.setDownloadListener { url, _, _, mimetype, _ ->

            val fileName: String = URLUtil.guessFileName(url, android.R.attr.contentDescription.toString(), mimetype)

            val request = DownloadManager.Request(Uri.parse(url)).setNotificationVisibility(DownloadManager.Request.VISIBILITY_VISIBLE_NOTIFY_COMPLETED) // Visibility of the download Notification
                .setDestinationInExternalPublicDir(Environment.DIRECTORY_DOWNLOADS, fileName).setTitle(fileName) // Title of the Download Notification
                .setDescription(context.getString(R.string.Downloading)) // Description of the Download Notification
                .setAllowedOverMetered(true) // Set if download is allowed on Mobile network
                .setAllowedOverRoaming(true) // Set if download is allowed on roaming network

            val downloadManager = context.getSystemService(DOWNLOAD_SERVICE) as DownloadManager?
            downloadID = downloadManager!!.enqueue(request) // enqueue puts the download request in the queue.
            context.registerReceiver(onDownloadComplete, IntentFilter(DownloadManager.ACTION_DOWNLOAD_COMPLETE))
           //  Snackbar.make(binding.root, R.string.download_started, Snackbar.LENGTH_SHORT).show()
        }
    }

    private val onDownloadComplete: BroadcastReceiver = object : BroadcastReceiver() {
        override fun onReceive(context: Context, intent: Intent) { //Fetching the download id received with the broadcast
            val id = intent.getLongExtra(DownloadManager.EXTRA_DOWNLOAD_ID, -1) //Checking if the received broadcast is for our enqueued download by matching download id
            if (id == downloadID) {
                Toast.makeText(context, context.getString(R.string.download_completed), Toast.LENGTH_SHORT).show()
            }
        }
    }

}