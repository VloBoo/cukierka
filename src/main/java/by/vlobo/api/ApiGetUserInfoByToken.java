package by.vlobo.api;

import org.json.JSONObject;

import by.vlobo.IApiProcessor;
import by.vlobo.App;

public class ApiGetUserInfoByToken implements IApiProcessor {

    @Override
    public JSONObject process(JSONObject message, App instance) {
        String token = message.getJSONObject("auth").toString();
        //String user = instance.getDatabase()
        return instance.getDatabase().getUserInfo(token);
    }
}
