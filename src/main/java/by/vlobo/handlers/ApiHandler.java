package by.vlobo.handlers;

import com.sun.net.httpserver.HttpHandler;
import com.sun.net.httpserver.HttpExchange;

import java.io.IOException;
import java.io.OutputStream;
import java.lang.reflect.InvocationTargetException;
import java.util.Set;

import org.json.JSONException;
import org.json.JSONObject;
import org.reflections.Reflections;

import by.vlobo.AApiProcessor;
import by.vlobo.App;
import by.vlobo.IApiProcessor;

public class ApiHandler implements HttpHandler {
    private App app;

    public ApiHandler(App app) {
        this.app = app;
    }

    @Override
    public void handle(HttpExchange t) {
        try {
            String test = new String(t.getRequestBody().readAllBytes());
            System.out.println(test);
            JSONObject jsonObject = new JSONObject(test);
            String response = processMessage(jsonObject.getString("key"), jsonObject);
            t.getResponseHeaders().set("Content-Type", "application/json; charset=UTF-8");
            t.sendResponseHeaders(200, response.length());
            try (OutputStream os = t.getResponseBody()) {
                os.write(response.getBytes());
            }
        } catch (JSONException | IOException e) {
            e.printStackTrace();
        }

    }

    private String processMessage(String key, JSONObject message) {
        Reflections reflections = new Reflections("by.vlobo.api");
        System.out.println(key);
        Set<Class<?>> annotatedClasses = reflections.getTypesAnnotatedWith(AApiProcessor.class);
        for (Class<?> clazz : annotatedClasses) {
            AApiProcessor annotation = clazz.getAnnotation(AApiProcessor.class);
            if (annotation.key().equals(key)) {
                try {
                    IApiProcessor processor = (IApiProcessor) clazz.getDeclaredConstructor().newInstance();
                    return processor.process(message, app);
                } catch (InstantiationException | IllegalAccessException | NoSuchMethodException
                        | InvocationTargetException e) {
                    e.printStackTrace();
                    return "{}";
                }
            }
        }

        System.err.println("No processor found for key: " + key);
        return "{}";
    }
}
