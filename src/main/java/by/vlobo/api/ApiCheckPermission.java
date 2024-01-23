package by.vlobo.api;

import org.json.JSONObject;

import by.vlobo.App;
import by.vlobo.Database;
import by.vlobo.IApiProcessor;
import by.vlobo.Tools;

public class ApiCheckPermission implements IApiProcessor {

    @Override
    public JSONObject process(JSONObject message, App instance, String user) {
        if (user == null) {
            return IApiProcessor.CODE_401_UNAUTHORIZED;
        }
        Database database = instance.getDatabase();
        JSONObject jsonObject = database.getPermission(user, message.getString("project"));
        if (jsonObject == null) {
            return IApiProcessor.CODE_500_INTERNAL_SERVER_ERROR;
        }
        return Tools.addJsonObject(IApiProcessor.CODE_200_OK, jsonObject);
    }
}
