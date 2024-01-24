package by.vlobo.api;

import org.json.JSONObject;

import by.vlobo.App;
import by.vlobo.IApiProcessor;
import by.vlobo.Tools;

public class ApiGetBugInfo implements IApiProcessor {

    @Override
     public JSONObject process(JSONObject message, App instance, String user) {
         if (user == null) {
             return IApiProcessor.CODE_401_UNAUTHORIZED;
         }
         JSONObject jsonObject = instance.getDatabase().getBugInfo(message.getString("bug"));
         if (jsonObject == null) {
             return IApiProcessor.CODE_500_INTERNAL_SERVER_ERROR;
         }
         return Tools.addJsonObject(IApiProcessor.CODE_200_OK, jsonObject);
     }
 }
 