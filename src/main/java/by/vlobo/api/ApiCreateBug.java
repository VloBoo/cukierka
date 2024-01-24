package by.vlobo.api;

import java.time.ZonedDateTime;

import org.json.JSONObject;

import by.vlobo.App;
import by.vlobo.Database;
import by.vlobo.IApiProcessor;
import by.vlobo.Tools;

public class ApiCreateBug implements IApiProcessor {

    @Override
    public JSONObject process(JSONObject message, App instance, String user) {
        Database database = instance.getDatabase();
        String bugName = message.getString("name");
        String bugDescription = message.getString("description");
        String projectId = message.getString("project");
        String criticality = message.getString("criticality");
        String priority = message.getString("priority");
        String status = "Open";
        String dateCreation = Tools.formatDateTime(ZonedDateTime.now());
        JSONObject other = new JSONObject();
        other.put("date of creation", dateCreation);
        String bugId = Tools.randomUUID2();

        JSONObject jsonObject = database.addBug(bugId, bugName, bugDescription, user, projectId, criticality,
                priority, status, other);

        if (jsonObject == null) {
            return IApiProcessor.CODE_500_INTERNAL_SERVER_ERROR;
        }

        return Tools.addJsonObject(IApiProcessor.CODE_200_OK, jsonObject);
    }
}