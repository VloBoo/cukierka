package by.vlobo.api;

import java.time.ZonedDateTime;

import org.json.JSONObject;

import by.vlobo.App;
import by.vlobo.Database;
import by.vlobo.IApiProcessor;
import by.vlobo.Tools;

public class ApiCreateUser implements IApiProcessor {

    @Override
    public JSONObject process(JSONObject message, App instance, String user) {
        Database database = instance.getDatabase();
        String username = message.getString("username");
        String email = message.getString("email");
        String password = Tools.hashPassword(message.getString("password"));
        String dateCreation = Tools.formatDateTime(ZonedDateTime.now());
        JSONObject other = new JSONObject();
        other.put("password", password);
        other.put("date of creation", dateCreation);
        JSONObject jsonObject = database.addUser(Tools.randomUUID2(), username, email, other);
        if (jsonObject == null) {
            return IApiProcessor.CODE_500_INTERNAL_SERVER_ERROR;
        }
        return Tools.addJsonObject(IApiProcessor.CODE_200_OK, jsonObject);
    }
}
