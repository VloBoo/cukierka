package by.vlobo.handlers;

import com.sun.net.httpserver.HttpHandler;
import com.sun.net.httpserver.HttpExchange;

import java.io.IOException;
import java.io.OutputStream;
import java.lang.reflect.InvocationTargetException;
import java.util.Set;

import org.reflections.Reflections;

import by.vlobo.ApiClass;
import by.vlobo.ApiProcessor;

public class ApiHandler implements HttpHandler {
    @Override
    public void handle(HttpExchange t) throws IOException {
        String response = processMessage("CheckDatabase", "null");
        t.getResponseHeaders().set("Content-Type", "application/json; charset=UTF-8");
        t.sendResponseHeaders(200, response.length());
        try (OutputStream os = t.getResponseBody()) {
            os.write(response.getBytes());
        }
    }

    static String processMessage(String key, String message) {
        Reflections reflections = new Reflections("by.vlobo.api"); 
        Set<Class<?>> annotatedClasses = reflections.getTypesAnnotatedWith(ApiClass.class);
        for (Class<?> clazz : annotatedClasses) {
            ApiClass annotation = clazz.getAnnotation(ApiClass.class);
            if (annotation.key().equals(key)) {
                try {
                    ApiProcessor processor = (ApiProcessor) clazz.getDeclaredConstructor().newInstance();
                    return processor.process(message);
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
