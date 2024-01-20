package by.vlobo;

import com.sun.net.httpserver.HttpExchange;

import java.io.File;
import java.io.IOException;
import java.io.OutputStream;
import java.lang.reflect.InvocationTargetException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.security.MessageDigest;
import java.security.NoSuchAlgorithmException;
import java.sql.ResultSet;
import java.sql.SQLException;
import java.time.ZonedDateTime;
import java.time.format.DateTimeFormatter;
import java.util.HashMap;
import java.util.HashSet;
import java.util.Map;
import java.util.UUID;
import java.util.regex.Pattern;
import java.util.stream.Collectors;

import org.json.JSONArray;
import org.json.JSONObject;
import org.jsoup.Jsoup;
import org.jsoup.nodes.Document;
import org.jsoup.nodes.Element;
import org.jsoup.select.Elements;

public class Tools {
    public static String hashPassword(String plainPassword) {
        try {
            MessageDigest messageDigest = MessageDigest.getInstance("SHA-256");
            byte[] hashBytes = messageDigest.digest(plainPassword.getBytes());

            // Преобразование байтов хеша в шестнадцатеричную строку
            StringBuilder stringBuilder = new StringBuilder();
            for (byte b : hashBytes) {
                stringBuilder.append(String.format("%02x", b));
            }

            return stringBuilder.toString();
        } catch (NoSuchAlgorithmException e) {
            e.printStackTrace();
            // Обработка ошибки
            return null;
        }
    }

    // Метод для проверки соответствия введенного пароля хешу
    public static boolean checkPassword(String userInputPassword, String hashedPassword) {
        String userInputHash = hashPassword(userInputPassword);
        return hashedPassword.equals(userInputHash);
    }

    public static String randomUUID2() {
        // Генерация UUID на основе имени (namespace-based UUID)
        String namespace = UUID.randomUUID().toString();
        String name = "cukierka";
        UUID nameBasedUUID = UUID.nameUUIDFromBytes((namespace + name).getBytes());
        return nameBasedUUID.toString();
    }

    public static JSONObject toJSONObject(ResultSet rs) {
        try {
            int columns = rs.getMetaData().getColumnCount();
            JSONObject jsonObject = new JSONObject();

            for (int i = 1; i <= columns; i++) {
                String columnName = rs.getMetaData().getColumnName(i);
                Object value = rs.getObject(i).toString();
                jsonObject.put(columnName, value);
            }
            return jsonObject;
        } catch (SQLException e) {
            e.printStackTrace();
            return null;
        }
    }

    public static String formatDateTime(ZonedDateTime zonedDateTime) {
        DateTimeFormatter formatter = DateTimeFormatter.ofPattern("yyyy-MM-dd'T'HH:mm:ssXXX");
        return zonedDateTime.format(formatter);
    }

    public static HashMap<Pattern, String> readRoadmap(JSONObject jsonObject) {
        HashMap<String, String> hashMap = new HashMap<String, String>();
        readRoadmapR(jsonObject, hashMap, "^");
        HashMap<Pattern, String> patternHashMap = new HashMap<>();
        for (Map.Entry<String, String> entry : hashMap.entrySet()) {
            String key = entry.getKey();
            String value = entry.getValue();
            Pattern pattern = Pattern.compile(key);
            patternHashMap.put(pattern, value);
        }
        return patternHashMap;
    }

    private static void readRoadmapR(JSONObject jsonObject, HashMap<String, String> result, String s) {
        switch (jsonObject.getInt("type")) {
            case 0: {// ничего
                s += jsonObject.getString("i") + "\\/";
                break;
            }
            case 1: {// обычный файл
                s += jsonObject.getString("i") + "\\/";
                result.put(s + "?$", jsonObject.getString("index"));
                break;
            }
            case 2: {// api
                s += String.format("(?<%s>[0-9a-zA-Z\\-]+)\\/", jsonObject.getString("i"));
                result.put(s + "?$", "#");
                break;
            }
            case 3: {// переменная
                s += String.format("(?<%s>[0-9a-zA-Z\\-]+)\\/", jsonObject.getString("i"));
                result.put(s + "?$", jsonObject.getString("index"));
                break;
            }
            case 4: {// cdn
                s += String.format("(?<%s>.*)", jsonObject.getString("i"));
                result.put(s + "$", "$");
                break;
            }
            default:
                return;
        }
        if (jsonObject.has("next")) {
            JSONArray nextValue = jsonObject.optJSONArray("next");
            for (Object object : nextValue) {
                JSONObject nextObject = (JSONObject) object;
                readRoadmapR(nextObject, result, new String(s));
            }
        }
    }

    public static JSONObject processMessage(String key, JSONObject message, App instance, String user) {

        try {
            Class<?> clazz = Class.forName("by.vlobo.api.Api" + key);
            System.out.println(key);

            if (IApiProcessor.class.isAssignableFrom(clazz)) {
                IApiProcessor processor = (IApiProcessor) clazz.getDeclaredConstructor().newInstance();;
                return processor.process(message, instance, user);
            }
        } catch (InstantiationException | IllegalAccessException | NoSuchMethodException
                | InvocationTargetException e) {
            e.printStackTrace();
            return IApiProcessor.CODE_500_INTERNAL_SERVER_ERROR;
        } catch (ClassNotFoundException e) {
            System.err.println("No processor found for key: " + key);
            return IApiProcessor.CODE_404_NOT_FOUND;
        }
        return IApiProcessor.CODE_500_INTERNAL_SERVER_ERROR;
    }

    public static JSONObject addJsonObject(JSONObject jo1s, JSONObject jo2s) {
        JSONObject jo = new JSONObject();
        JSONObject jo1 = new JSONObject(jo1s.toString());
        JSONObject jo2 = new JSONObject(jo2s.toString());
        for (String key : jo1.keySet()) {
            jo.put(key, jo1.get(key));
        }
        for (String key : jo2.keySet()) {
            jo.put(key, jo2.get(key));
        }
        return jo;
    }

    public static void sendString(HttpExchange t, int code, String str) throws IOException {
        byte[] bstr = str.getBytes("UTF-8");
        t.sendResponseHeaders(code, bstr.length);
        try (OutputStream os = t.getResponseBody()) {
            os.write(bstr);
        }
    }

    public static void sendFile(HttpExchange t, int code, File file) throws IOException {
        t.sendResponseHeaders(code, file.length());
        try (OutputStream os = t.getResponseBody()) {
            Files.copy(file.toPath(), os);
        }
    }

    public static String linkHtml(String firstHtml, HashMap<String, String> vars) throws IOException {

        HashSet<String> cssFiles = new HashSet<String>();
        HashSet<String> jsFiles = new HashSet<String>();
        HashMap<String, Float> jsMain = new HashMap<String, Float>();
        Document doc = processInclude(firstHtml, cssFiles, jsFiles, jsMain);

        StringBuilder cssCommon = new StringBuilder();
        StringBuilder jsCommon = new StringBuilder();

        for (String fileName : cssFiles) {
            try {
                String cssContent = Files.readString(Path.of("www/" + fileName));
                cssCommon.append(cssContent).append("\n");
            } catch (IOException e) {
                e.printStackTrace();
            }
        }

        jsCommon.append(String.format("let global = %s;\n", new JSONObject(vars).toString()));

        for (String fileName : jsFiles) {
            try {
                String jsContent = Files.readString(Path.of("www/" + fileName));
                jsCommon.append(jsContent).append("\n");
            } catch (IOException e) {
                e.printStackTrace();
            }
        }
        // document.addEventListener("DOMContentLoaded", function () {});
        for (String fun : jsMain.entrySet()
                .stream()
                .sorted(Map.Entry.comparingByValue())
                .map(Map.Entry::getKey)
                .collect(Collectors.toList())) {
            jsCommon.append(fun).append("\n");
        }

        doc.head().append("<style>\n" + cssCommon.toString() + "\n</style>");
        doc.body().append("<script>\n" + jsCommon.toString() + "\n</script>");
        //System.out.println(doc.toString());
        return doc.toString();
    }

    private static Document processInclude(
            String filePath,
            HashSet<String> cssFiles,
            HashSet<String> jsFiles,
            HashMap<String, Float> jsMain)
            throws IOException {
        File inputFile = new File("www/" + filePath);
        Document doc = Jsoup.parse(inputFile, "UTF-8", "");

        Elements requiredEls = doc.select("required");
        for (Element includeEl : requiredEls.select("[css]")) {
            String srcValue = includeEl.attr("src");
            cssFiles.add(srcValue);
            includeEl.remove();
        }
        for (Element includeEl : requiredEls.select("[js]")) {
            String srcValue = includeEl.attr("src");
            jsFiles.add(srcValue);
            includeEl.remove();
        }
        for (Element includeEl : requiredEls.select("[jsmain]")) {
            String srcValue = includeEl.attr("src");
            Float priorValue = Float.parseFloat(includeEl.attr("jsmain"));
            jsMain.put(srcValue, priorValue);
            includeEl.remove();
        }

        Elements includeEls = doc.select("include[src]");
        for (Element includeEl : includeEls) {
            String srcValue = includeEl.attr("src");
            String includedContent = processInclude(srcValue, cssFiles, jsFiles, jsMain).body().html();
            includeEl.after(includedContent);
            includeEl.remove();
        }

        return doc;
    }
}
