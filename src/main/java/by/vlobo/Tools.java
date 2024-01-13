package by.vlobo;

import java.security.MessageDigest;
import java.security.NoSuchAlgorithmException;
import java.sql.ResultSet;
import java.sql.SQLException;
import java.time.ZonedDateTime;
import java.time.format.DateTimeFormatter;
import java.util.UUID;

import org.json.JSONObject;

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
}
