package com.app.webapp
//<?xml version="1.0" encoding="utf-8"?>
//<resources>
//    <item name="site_url" type="string">www.google.com</item>
//    <item name="aboutUs" type="string">aboutUSTEXT</item>
//    <item name="splash_screen" type="integer">1</item>
//<!--    There is multi type of splash screen just put the number of what you want instead of 1-->
//
//<!--    0-No splash screen-->
//<!--    1-Just icon and app name-->
//<!--    2-Icon with animation-->
//<!--    3-full screen image-->
//<!--    4-icon and app name with gradient color1-->
//    <item name="splash_screen" type="integer">1</item>
//<!--    gradient color for splash-->
//    <item name="SplashScreenGC" type="integer">1</item>
//<!--    There are multi type of cacheMode-->
//<!--    1-LOAD_DEFAULT (Default cache usage mode. If the navigation type doesn't impose any specific behavior, use cached resources when they are available and not expired, otherwise load resources from the network)-->
//<!--    2-LOAD CACHE ELSE NETWORK (Use cached resources when they are available, even if they have expired. Otherwise load resources from the network.)-->
//<!--    number 1 is recommended-->
//    <item name="cacheMode" type="integer">1</item>
//<!--    There are multi layout of no Internet-->
//<!--    1-simple with image and a text-->
//<!--    2-with lottie animation and a text-->
//    <item name="no_internet_layout" type="integer">1</item>
//<!--    You can have different type of toolbar-->
//<!--    0- no toolbar-->
//<!--    1- toolbar with text-->
//<!--    2- toolbar with image TODO add image-->
//    <item name="toolbar" type="integer">1</item>
//<!--    You can have icon that show url in web view when you click on it(it used for cart or FAG pages)-->
//<!--    EX: Pair( R.drawable.ICON_NAME , URL LINK )-->
//<!--    if you don't want just simply write "null"-->
//    <item name="toolbar_custom_icon" type="integer">1</item>
//<!--    you can have sidebar menu (if you want to have menu its better to have toolbar too)-->
//<!--    0- don't have menu-->
//<!--    1- have menu-->
//    <item name="sidebar_menu" type="integer">1</item>
//<!--    you can change header on slider-->
//<!--    0- no header-->
//<!--    1- header with gradient-->
//    <item name="sidebar_menu_header_mode" type="integer">1</item>
//<!--    select your gradient color for menu header-->
//    <item name="sidebar_menu_header_color" type="integer">1</item>
//<!--    you can change footer on slider-->
//<!--    0- no footer-->
//<!--    1- with footer (you can change the text in strings.xml )-->
//    <item name="sidebar_menu_footer_mode" type="integer">1</item>
//<!--    you can have Swipe Refresh page-->
//<!--    0-don't have-->
//<!--    1- have-->
//    <item name="swipe_refresh" type="integer">1</item>
//<!--    ask for camera permission-->
//<!--    if yes write true if not write false-->
//    <item name="camera_permission" type="integer">1</item>
//<!--    ask for location permission-->
//<!--    if you need it write  "true" if not write "false"-->
//    <item name="location_permission" type="integer">1</item>
//<!--    ask for microphone permission-->
//<!--    if you need it write  "true" if not write "false"-->
//    <item name="microphone_permission" type="integer">1</item>
//<!--    add admob-->
//<!--    if you need it write  "true" if not write "false"-->
//    <item name="admob" type="integer">1</item>
//<!--    you can show admob banner and (or) admob Interstitial-->
//<!--    if you need it write  "true" if not write "false"-->
//    <item name="admob_banner" type="integer">1</item>
//<!--    floating action button menu-->
//<!--    if you need it write  "true" if not write "false"-->
//    <item name="floating_action_button_menu" type="integer">1</item>
//</resources>

import android.annotation.SuppressLint
import android.content.Context
import kotlinx.serialization.json.Json
import java.io.BufferedReader
import java.io.InputStream
import java.io.InputStreamReader
import java.io.Reader


open class Config(private val context: Context) {

    public var configType: dataType;
    @kotlinx.serialization.Serializable
    data class dataType(
        val admob: Int,
        val admob_banner: Int,
        val cache_mode: Int,
        val floating_action_button_menu: Int,
        val no_internet_layout: Int,
        val sidebar_menu: Int,
        val sidebar_menu_footer_mode: Int,
        val sidebar_menu_header_color: Int,
        val sidebar_menu_header_mode: Int,
        val site_url: String,
        val splash_screen: Int,
        val splash_screen_g_c: Int,
        val swipe_refresh: Int,
        val toolbar: Int,
        val toolbar_custom_icon: Pair<String , String>? = null,
        val about_us: String? = null,
        val item_menu: List<MenuItem>,
        val item_fab:List<MenuItem>,
        val intro_pages: List<IntroPage>,
        val introPage: Boolean
    )
    @kotlinx.serialization.Serializable
data class MenuItem(
    val Kind: Int? = null,
    val Pair: Pair<String,String>? = null
)
    @kotlinx.serialization.Serializable
data class IntroPage(
    val background: Int,
    val description: String,
    val image_name: String,
    val title: String
) {
    fun background_grident(): Int {
        return when (this.background) {
            1 -> R.drawable.bg_gradient_slide1
            2 -> R.drawable.bg_gradient_slide2
            3 -> R.drawable.bg_gradient_slide3
            4 -> R.drawable.bg_gradient_slide4
            else -> R.drawable.bg_gradient_slide5
        }
    }
}

    init {
       // val gson = Gson()
        val raw: InputStream = context.resources.openRawResource(R.raw.setting)
        val rd: Reader = BufferedReader(InputStreamReader(raw))
        configType = Json{
            ignoreUnknownKeys = true
        }.decodeFromString(rd.readText())
        rd.close()
    }


    companion object {
        @SuppressLint("StaticFieldLeak")
        @Volatile
        private var INSTANCE: Config? = null
        fun getInstance(context: Context): Config =
            INSTANCE ?: synchronized(this) {
                INSTANCE ?: Config(context).also { INSTANCE = it }
            }
        /*
    intro Page
    if you need it write "true" if not write "false"
 */

//        const val introPage: Boolean = false

//        /*
//     intro items  (if its enabled)
//     you just have to write IntroItemModel(YOUR TITLE, YOUR DESCRIPTION, R.drawable.YOUR_IMAGE_NAME)
//     waring: you can add maximum 5 of this intro
//  */
//        val itemIntro = arrayListOf(
//            IntroItemModel(
//                "Secure",
//                "with cauliflowers drink iced tea. ",
//                R.drawable.shield,
//                1
//            ),
//            IntroItemModel("Fast", "where is the evasive processor.", R.drawable.fast_time, 2),
//            IntroItemModel("Easy to use", "why does the teleporter experiment.", R.drawable.snap, 3)
//        )
//
//        val ToolbarCustomIcon: Pair<Int, String>? =
//            Pair(R.drawable.ic_baseline_shopping_cart_24, "https://www.shopping.com")
//
//
//        /*
//            you can chose what item shows in menu
//            1-Home page
//            2-About us page
//            3-Rate us
//            4-Share app
//            5-Exit
//             custom url (you can add as many as you want)
//
//            you have to put number in order that you want
//              and for custom url you have to write Pair("NAME IS GOING TO SHOW" , "THE URL IT SHOULD LOAD" )
//
//            EX: (1, 2, Pair( "FAG", "www.example.com/FAG") , 3, 4, 5) it means the first item is home page after that it about us page after it is custom url
//                and after that it rate and after it if share app .....
//         */
//        var items = arrayListOf(1, 2, Pair("FAG", "www.example.com/FAG"), 3, 4, 5)
//
//
//        /*
//            firebase Analytics
//            if you need it write  "true" if not write "false"
//         */
//        const val firebaseAnalytics: Boolean = true
//
//
//        /*
//          floating action button menu items (if its enabled)
//            1-Home page
//            2-About us page
//            3-Rate us
//            4-Share app
//            custom url (you can add as many as you want)
//            for adding you just have to write item's number
//                and for custom url you have to write Pair("NAME IS GOING TO SHOW" , "THE URL IT SHOULD LOAD" )
//            EX: (1, 2, Pair( "FAG", "www.example.com/FAG") , 4 )
//       */
//        val itemFab =
//            arrayListOf(1, 2, Pair("FAG", "https://github.com/nambicompany/expandable-fab"))
//

    }

}
