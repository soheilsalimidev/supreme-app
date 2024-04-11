import glob
import re

json = "["
for name in glob.glob('./src/**/*.vue', recursive= True): 
    f = open(name, "r")
    imtag = re.search(r'<i18n>[\w\W]*</i18n>', f.read())
    if imtag:
        json = json + imtag.group(0).replace("<i18n>" , "").replace("</i18n>" , "") + ","

enL = open("./src/locales/en.json", "r")
faL = open("./src/locales/fa.json", "r")
open("./local.json", "w").write(json + "{\"en\":"+ enL.read() + ",\"fa\":" + faL.read() + "}]")
