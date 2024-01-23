package by.vlobo.api;

import org.json.JSONException;
import org.json.JSONObject;

import by.vlobo.IApiProcessor;
import by.vlobo.Tools;
import by.vlobo.App;

public class ApiGetProjectInfo implements IApiProcessor {

    @Override
    public JSONObject process(JSONObject message, App instance, String user) {
        if (user == null) {
            return IApiProcessor.CODE_401_UNAUTHORIZED;
        }
        String id;
        try {
            id = message.getString("id");
        } catch (JSONException e) {
            return IApiProcessor.CODE_400_BAD_REQUEST;
        }

        JSONObject jsonObject = instance.getDatabase().getProjectInfo(id);
        if (jsonObject == null) {
            return IApiProcessor.CODE_500_INTERNAL_SERVER_ERROR;
        }
        return Tools.addJsonObject(IApiProcessor.CODE_200_OK, jsonObject);
    }
}
