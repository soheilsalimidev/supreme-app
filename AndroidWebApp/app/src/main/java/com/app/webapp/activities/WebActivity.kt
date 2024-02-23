package com.app.webapp.activities

import android.Manifest
import android.annotation.SuppressLint
import android.content.ActivityNotFoundException
import android.content.Intent
import android.content.pm.PackageManager
import android.net.Uri
import android.os.Bundle
import android.util.Log
import android.view.KeyEvent
import android.view.Menu
import android.view.MenuItem
import android.view.View
import android.webkit.*
import androidx.activity.result.contract.ActivityResultContracts
import androidx.appcompat.app.AppCompatActivity
import androidx.core.app.ActivityCompat
import androidx.core.content.ContextCompat
import androidx.core.content.res.ResourcesCompat
import com.afollestad.materialdialogs.MaterialDialog
import com.app.webapp.Config
import com.app.webapp.R
import com.app.webapp.databinding.ActivityWebBinding
import com.app.webapp.databinding.SliderHeaderBinding
import com.app.webapp.utility.MyWebViewClient
//import com.google.firebase.FirebaseApp
//import com.google.firebase.FirebaseOptions
//import com.google.firebase.analytics.FirebaseAnalytics
import com.mikepenz.materialdrawer.model.PrimaryDrawerItem
import com.mikepenz.materialdrawer.model.interfaces.iconRes
import com.mikepenz.materialdrawer.model.interfaces.nameRes
import com.mikepenz.materialdrawer.model.interfaces.nameText
import com.mikepenz.materialdrawer.util.addStickyFooterItem
import com.nambimobile.widgets.efab.FabOption
import java.util.*


@SuppressLint("StaticFieldLeak")
lateinit var binding: ActivityWebBinding

class WebActivity : AppCompatActivity() {

    private var uploadMessage: ValueCallback<Array<Uri?>?>? = null
//    private lateinit var firebaseAnalytics: FirebaseAnalytics

    val resultLauncher =
        registerForActivityResult(ActivityResultContracts.StartActivityForResult()) { result ->
            val data: Intent = result.data!!
            if (uploadMessage != null) {
                uploadMessage!!.onReceiveValue(
                    WebChromeClient.FileChooserParams.parseResult(
                        result.resultCode,
                        data
                    )
                )
                uploadMessage = null
            }
        }
    private lateinit var config: Config
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        binding = ActivityWebBinding.inflate(layoutInflater)
        setContentView(binding.root)
        config = Config.getInstance(this);
        val data: String? = intent?.data?.host
        Log.i("TAG", "onCreate: $data")
        binding.webView.loadUrl((data ?: config.configType.siteUrl).toString())

        layoutSetting()

        webSetting()

//        val builder = FirebaseOptions.Builder()
//            .setApplicationId("1:0123456789012:android:0123456789abcdef")
//            .setApiKey("your_api_key")
//        FirebaseApp.initializeApp(this, builder.build())
//        if (config.configType.admob as Boolean) admobSetting()
//        if (Config.firebaseAnalytics) {
//            Firebase.analytics.setAnalyticsCollectionEnabled(true)
//            firebaseAnalytics = Firebase.analytics
//        }


    }

//    private fun admobSetting() {
//        MobileAds.initialize(this) {}
//        val adRequest = AdRequest.Builder().build()
//        if (config.configType.admob as Boolean) {
//            binding.adView.visibility = View.VISIBLE
//            binding.adView.loadAd(adRequest)
//        }
//
//        if (Config.admobInterstitial) InterstitialAd.load(this, "ca-app-pub-4467163011960707/2951594956", AdRequest.Builder().build(), object : InterstitialAdLoadCallback() {
//            override fun onAdFailedToLoad(adError: LoadAdError) {
//                Log.d("TAG", adError.message)
//            }
//
//            override fun onAdLoaded(interstitialAd: InterstitialAd) {
//                Log.d("TAG", "Ad was loaded.")
//                interstitialAd.show(this@WebActivity)
//            }
//        })
//    }

    @SuppressLint("UseCompatLoadingForDrawables")
    private fun layoutSetting() {

        if (config.configType.toolbar.type == 1) {
//            binding.topAppBar.visibility = View.VISIBLE
//            setSupportActionBar(binding.toolbar)
        } else binding.customToolbar.visibility = View.VISIBLE


        if (config.configType.sidebarMenu.enable) menuSetup()

        if (config.configType.swipeRefresh) binding.swipeRefreshLayout.isEnabled = false
        else binding.swipeRefreshLayout.setOnRefreshListener {
            binding.webView.reload()
        }

//        if (config.configType.toolbar_custom_icon != null && config.configType.toolbar == 2) {
//            binding.custom.setImageResource(config.configType.toolbar_custom_icon.first)
//            binding.custom.setOnClickListener {
//                binding.webView.loadUrl(config.configType.toolbar_custom_icon.second)
//            }
//        }

        if (config.configType.floatingActionButton.enable) {
            binding.fabLayout.visibility = View.VISIBLE
            config.configType.floatingActionButton.itemFab.forEach { item ->
                val fab = FabOption(this)
                fab.label.translationXPx = (-100).toFloat()
                when (item.kind) {
                    1 -> {
                        fab.label.labelText = getString(R.string.home_page)
                        fab.fabOptionIcon = getDrawable(R.drawable.ic_baseline_home_24)
                    }

                    2 -> {
                        fab.label.labelText = getString(R.string.about_us)
                        fab.fabOptionIcon = getDrawable(R.drawable.ic_baseline_groups_24)
                        fab.setOnClickListener {
                            startActivity(Intent(this@WebActivity, AboutUsActivity::class.java))
                        }
                    }

                    3 -> {
                        fab.label.labelText = getString(R.string.rate_us)
                        fab.fabOptionIcon = getDrawable(R.drawable.ic_baseline_star_rate_24)
                        fab.setOnClickListener {
                            val uri: Uri = Uri.parse("market://details?id=$packageName")
                            val goToMarket = Intent(Intent.ACTION_VIEW, uri)
                            goToMarket.addFlags(Intent.FLAG_ACTIVITY_NO_HISTORY or Intent.FLAG_ACTIVITY_NEW_DOCUMENT or Intent.FLAG_ACTIVITY_MULTIPLE_TASK)
                            try {
                                startActivity(goToMarket)
                            } catch (e: ActivityNotFoundException) {
                                startActivity(
                                    Intent(
                                        Intent.ACTION_VIEW,
                                        Uri.parse("http://play.google.com/store/apps/details?id=$packageName")
                                    )
                                )
                            }
                        }
                    }

                    4 -> {
                        fab.label.labelText = getString(R.string.share_app)
                        fab.fabOptionIcon = getDrawable(R.drawable.ic_baseline_share_24)
                        fab.setOnClickListener {
                            val sendIntent = Intent()
                            sendIntent.action = Intent.ACTION_SEND
                            sendIntent.putExtra(
                                Intent.EXTRA_TEXT,
                                getString(R.string.checkOutTheApp) + config.configType.siteUrl
                            )
                            sendIntent.type = "text/plain"
                            startActivity(sendIntent)
                        }
                    }

                    else -> {
                        val pair = item.pair
                        fab.label.labelText = pair?.first
                        fab.fabOptionIcon = getDrawable(R.drawable.ic_baseline_extension_24)
                        fab.setOnClickListener {
                            pair?.second?.let { it1 -> binding.webView.loadUrl(it1) }
                            binding.fabLayout.close()
                        }
                    }
                }
                binding.fabLayout.addView(fab)
            }
        }
    }

    private fun menuSetup() {

        config.configType.sidebarMenu.itemMenu.forEach { item ->
            when (item.kind) {
                1 -> binding.slider.itemAdapter.add(PrimaryDrawerItem().apply {
                    nameRes = R.string.home_page; identifier = (item.kind).toLong(); iconRes =
                    R.drawable.ic_baseline_home_24
                    typeface = if (Locale.getDefault().language == "fa") ResourcesCompat.getFont(
                        this@WebActivity,
                        R.font.iransansmobile
                    ) else ResourcesCompat.getFont(this@WebActivity, R.font.roboto_regular)
                })

                2 -> binding.slider.itemAdapter.add(PrimaryDrawerItem().apply {
                    nameRes = R.string.about_us; identifier = (item.kind).toLong(); iconRes =
                    R.drawable.ic_baseline_groups_24
                    typeface = if (Locale.getDefault().language == "fa") ResourcesCompat.getFont(
                        this@WebActivity,
                        R.font.iransansmobile
                    ) else ResourcesCompat.getFont(this@WebActivity, R.font.roboto_regular)
                })

                3 -> binding.slider.itemAdapter.add(PrimaryDrawerItem().apply {
                    nameRes = R.string.rate_us; identifier = (item.kind).toLong(); iconRes =
                    R.drawable.ic_baseline_star_rate_24
                    typeface = if (Locale.getDefault().language == "fa") ResourcesCompat.getFont(
                        this@WebActivity,
                        R.font.iransansmobile
                    ) else ResourcesCompat.getFont(this@WebActivity, R.font.roboto_regular)
                })

                4 -> binding.slider.itemAdapter.add(PrimaryDrawerItem().apply {
                    nameRes = R.string.share_app; identifier = (item.kind).toLong(); iconRes =
                    R.drawable.ic_baseline_share_24
                    typeface = if (Locale.getDefault().language == "fa") ResourcesCompat.getFont(
                        this@WebActivity,
                        R.font.iransansmobile
                    ) else ResourcesCompat.getFont(this@WebActivity, R.font.roboto_regular)
                })

                5 -> binding.slider.itemAdapter.add(PrimaryDrawerItem().apply {
                    nameRes = R.string.exit; identifier = (item.kind).toLong(); iconRes =
                    R.drawable.ic_baseline_exit_to_app_24
                    typeface = if (Locale.getDefault().language == "fa") ResourcesCompat.getFont(
                        this@WebActivity,
                        R.font.iransansmobile
                    ) else ResourcesCompat.getFont(this@WebActivity, R.font.roboto_regular)
                })

                else -> {
                    val pair = item.pair
                    binding.slider.itemAdapter.add(PrimaryDrawerItem().apply {
                        nameText = pair?.first.toString(); iconRes =
                        R.drawable.ic_baseline_extension_24
                        typeface =
                            if (Locale.getDefault().language == "fa") ResourcesCompat.getFont(
                                this@WebActivity,
                                R.font.iransansmobile
                            ) else ResourcesCompat.getFont(this@WebActivity, R.font.roboto_regular)
                        onDrawerItemClickListener = { _, _, _ ->
                            binding.webView.loadUrl(pair?.second.toString())
                            true
                        }
                    })
                }
            }
        }

        binding.slider.selectedItemIdentifier = 1

        if (config.configType.sidebarMenu.sidebarMenuHeader.type == 1) {
            val sliderHeaderBinding: SliderHeaderBinding =
                SliderHeaderBinding.inflate(layoutInflater)
            sliderHeaderBinding.background.setBackgroundResource(
                R.drawable.bg_gradient_menu_header
            )
            binding.slider.headerView = sliderHeaderBinding.root
        }

        if (config.configType.sidebarMenu.sidebarMenuFooter.type == 1) binding.slider.addStickyFooterItem(
            PrimaryDrawerItem().apply {
                nameRes = R.string.menu_footer_text
                typeface = if (Locale.getDefault().language == "fa") ResourcesCompat.getFont(
                    this@WebActivity,
                    R.font.iransansmobile
                ) else ResourcesCompat.getFont(this@WebActivity, R.font.roboto_regular)
            })
        binding.slider.onDrawerItemClickListener = { _, _, itemPosition ->
            when (itemPosition) {
                2 -> startActivity(Intent(this@WebActivity, AboutUsActivity::class.java))
                3 -> {
                    val uri: Uri = Uri.parse("market://details?id=$packageName")
                    val goToMarket = Intent(Intent.ACTION_VIEW, uri)
                    goToMarket.addFlags(Intent.FLAG_ACTIVITY_NO_HISTORY or Intent.FLAG_ACTIVITY_NEW_DOCUMENT or Intent.FLAG_ACTIVITY_MULTIPLE_TASK)
                    try {
                        startActivity(goToMarket)
                    } catch (e: ActivityNotFoundException) {
                        startActivity(
                            Intent(
                                Intent.ACTION_VIEW,
                                Uri.parse("http://play.google.com/store/apps/details?id=$packageName")
                            )
                        )
                    }
                }

                4 -> {
                    val sendIntent = Intent()
                    sendIntent.action = Intent.ACTION_SEND
                    sendIntent.putExtra(
                        Intent.EXTRA_TEXT,
                        getString(R.string.checkOutTheApp) + config.configType.siteUrl
                    )
                    sendIntent.type = "text/plain"
                    startActivity(sendIntent)
                }
            }
            true
        }

//        binding.toolbar.setNavigationOnClickListener {
//            binding.slider.drawerLayout?.openDrawer(binding.slider)
//        }

        binding.menu.setOnClickListener {
            binding.slider.drawerLayout?.openDrawer(binding.slider)
        }

    }

    @SuppressLint("SetJavaScriptEnabled")
    fun webSetting() {

        binding.webView.webViewClient = MyWebViewClient(this)

        val webSettings: WebSettings = binding.webView.settings
        webSettings.domStorageEnabled = true
        webSettings.displayZoomControls = false
        webSettings.useWideViewPort = true
        webSettings.javaScriptEnabled = true
        webSettings.cacheMode =
            if (config.configType.cacheMode == 1) WebSettings.LOAD_DEFAULT else WebSettings.LOAD_CACHE_ELSE_NETWORK
        webSettings.setGeolocationEnabled(true)
        webSettings.allowFileAccess = true
        webSettings.allowContentAccess = true
        webSettings.loadsImagesAutomatically = true
        binding.webView.scrollBarStyle = View.SCROLLBARS_INSIDE_OVERLAY
        webSettings.mediaPlaybackRequiresUserGesture = false

        handleWebChromeClient()

//        if (config.configType.cameraPermission as Boolean) cameraPermission()
//        if (config.configType.microphonePermission as Boolean) microphonePermission()
//        if (config.configType.locationPermission as Boolean) locationPermission()


    }

    private fun handleWebChromeClient() {
        binding.webView.webChromeClient = object : WebChromeClient() {

            override fun onShowFileChooser(
                mWebView: WebView?,
                filePathCallback: ValueCallback<Array<Uri?>?>,
                fileChooserParams: FileChooserParams
            ): Boolean {
                if (uploadMessage != null) {
                    uploadMessage!!.onReceiveValue(null)
                    uploadMessage = null
                }

                uploadMessage = filePathCallback

                val intent = fileChooserParams.createIntent()
                resultLauncher.launch(intent)
                return true
            }

            override fun onPermissionRequest(request: PermissionRequest?) {
                request?.grant(request.resources)
                Log.i("TAG", "onPermissionRequest: " + request.toString())
            }

            override fun onGeolocationPermissionsShowPrompt(
                origin: String?,
                callback: GeolocationPermissions.Callback
            ) {
                callback.invoke(origin, true, false)
            }

            override fun onJsAlert(
                view: WebView?,
                url: String?,
                message: String?,
                result: JsResult?
            ): Boolean {
                MaterialDialog(this@WebActivity).show {
                    title(text = getString(R.string.app_name) + " " + getString(R.string.says) + ":")
                    message(text = message)
                    positiveButton(R.string.ok)
                }
                return true
            }
        }
    }

    override fun onKeyDown(keyCode: Int, event: KeyEvent): Boolean {
        if (event.action == KeyEvent.ACTION_DOWN) {
            when (keyCode) {
                KeyEvent.KEYCODE_BACK -> {
                    if (binding.webView.canGoBack()) binding.webView.goBack()
                    else finish()
                    return true
                }
            }
        }
        return super.onKeyDown(keyCode, event)
    }

    private fun locationPermission() {
        if (ContextCompat.checkSelfPermission(
                this,
                Manifest.permission_group.LOCATION
            ) == PackageManager.PERMISSION_GRANTED
        ) ActivityCompat.requestPermissions(
            this,
            arrayOf(
                Manifest.permission.ACCESS_FINE_LOCATION,
                Manifest.permission.ACCESS_COARSE_LOCATION
            ),
            1
        )
    }

    private fun cameraPermission() {
        if (ContextCompat.checkSelfPermission(
                this,
                Manifest.permission.CAMERA
            ) == PackageManager.PERMISSION_GRANTED
        ) ActivityCompat.requestPermissions(this, arrayOf(Manifest.permission.CAMERA), 2)
    }

    private fun microphonePermission() {
        if (ContextCompat.checkSelfPermission(
                this,
                Manifest.permission.RECORD_AUDIO
            ) == PackageManager.PERMISSION_GRANTED
        ) ActivityCompat.requestPermissions(this, arrayOf(Manifest.permission.RECORD_AUDIO), 3)
    }

    override fun onRequestPermissionsResult(
        requestCode: Int,
        permissions: Array<out String>,
        grantResults: IntArray
    ) {
        super.onRequestPermissionsResult(requestCode, permissions, grantResults)
        if (grantResults[0] == PackageManager.PERMISSION_GRANTED) MaterialDialog(this).show {
            title(R.string.missing_permission)
            message(R.string.missing_permission_meg)
            positiveButton(R.string.ok)
        }
    }

    override fun onCreateOptionsMenu(menu: Menu?): Boolean {
//TODO:add this
//        if (config.configType.toolbarCustomIcon.enable) menu?.add(0, 0, 0, "")?.setIcon(config.configType.toolbarCustomIcon.first)?.setShowAsAction(MenuItem.SHOW_AS_ACTION_ALWAYS)
        return true
    }

}