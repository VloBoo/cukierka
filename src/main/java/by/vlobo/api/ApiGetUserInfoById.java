package by.vlobo.api;

import org.json.JSONObject;

import by.vlobo.IApiProcessor;
import by.vlobo.App;

public class ApiGetUserInfoById implements IApiProcessor {

    @Override
    public JSONObject process(JSONObject message, App instance, String user) {

        return IApiProcessor.CODE_501_NOT_IMPLEMENTED;
    }
}
