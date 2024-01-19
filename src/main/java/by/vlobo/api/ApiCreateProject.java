package by.vlobo.api;

import java.time.ZonedDateTime;

import org.json.JSONObject;

import by.vlobo.App;
import by.vlobo.Database;
import by.vlobo.IApiProcessor;
import by.vlobo.Tools;

public class ApiCreateProject implements IApiProcessor {

    @Override
    public JSONObject process(JSONObject message, App instance, String user) {
        if (user == null) {
            return IApiProcessor.CODE_401_UNAUTHORIZED;
        }
        Database database = instance.getDatabase();
        JSONObject other = new JSONObject();
        other.put("date of creation", Tools.formatDateTime(ZonedDateTime.now()));
        JSONObject jsonObject = database.addProject(user, message.getString("name"), other);
        if (jsonObject == null) {
            return IApiProcessor.CODE_500_INTERNAL_SERVER_ERROR;
        }
        if (database.addUsersToProjects(user, jsonObject.getString("id"), "owner") == null) {
            return IApiProcessor.CODE_500_INTERNAL_SERVER_ERROR;
        }
        return Tools.addJsonObject(IApiProcessor.CODE_200_OK, jsonObject);
    }
}
