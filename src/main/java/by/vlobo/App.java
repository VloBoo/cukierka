package by.vlobo;

import java.io.FileNotFoundException;
import java.io.FileReader;
import java.util.HashMap;
import java.util.regex.Pattern;

import org.json.JSONException;
import org.json.JSONObject;
import org.json.JSONTokener;

public class App {
    private Server server;
    private Database database;
    private HashMap<Pattern, String> roadmap;

    public App() {
        try {
            JSONObject jsonObject = new JSONObject(new JSONTokener(new FileReader("./www/roadmap.json")));
            roadmap = Tools.readRoadmap(jsonObject);
            System.out.println("Roadmap собран");
        } catch (JSONException e) {
            e.printStackTrace();
        } catch (FileNotFoundException e) {
            e.printStackTrace();
        }
        database = new Database("VloBo", " 1");
    }

    public void start() {
        server = new Server(this);
    }

    public HashMap<Pattern, String> getRoadmap() {
        return roadmap;
    }

    public Server getServer() {
        return server;
    }

    public Database getDatabase() {
        return database;
    }
}
