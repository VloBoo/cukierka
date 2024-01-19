package by.vlobo.api;

import org.json.JSONObject;

import by.vlobo.App;
import by.vlobo.IApiProcessor;
import by.vlobo.Tools;
import by.vlobo.Database;

public class ApiCheckDatabase implements IApiProcessor {

    @Override
    public JSONObject process(JSONObject message, App instance, String user) {
        Database database = instance.getDatabase();
        JSONObject jsonObject = database.basecheck();
        if (jsonObject == null) {
            return IApiProcessor.CODE_500_INTERNAL_SERVER_ERROR;
        }
        return Tools.addJsonObject(IApiProcessor.CODE_200_OK, jsonObject);
    }
}
