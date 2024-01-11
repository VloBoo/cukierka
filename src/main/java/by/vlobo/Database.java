package by.vlobo;

import java.sql.Connection;
import java.sql.DriverManager;
import java.sql.PreparedStatement;
import java.sql.ResultSet;
import java.sql.SQLException;
import java.sql.Statement;
import java.util.Properties;

import org.json.JSONArray;
import org.json.JSONObject;

// Этот класс должен служить прослойкой между бэком, что бы я спокойно вносил правки в бд и тут, а само приложение не трогал.
public class Database {
    private Connection connection;

    public static void preset() {
        try {
            Class.forName("org.postgresql.Driver");
        } catch (ClassNotFoundException e) {
            System.err.println("Dont found class name");
            e.printStackTrace();
        }
    }

    public Database(String user, String password, String host, String dbName) {
        try {
            String url = "jdbc:postgresql://" + host + "/" + dbName;
            Properties props = new Properties();
            props.setProperty("user", user);
            props.setProperty("password", password);
            connection = DriverManager.getConnection(url, props);
        } catch (SQLException e) {
            e.printStackTrace();
        }
    }

    public Database(String user, String password) {
        this(user, password, "localhost", "cukierka");
    }

    public JSONObject basecheck() {
        try {
            Statement st = connection.createStatement();
            ResultSet rs = st.executeQuery("SELECT \n" + //
                    "    table_name as \"Table\",\n" + //
                    "    column_name as \"Column\"\n" + //
                    "FROM \n" + //
                    "    information_schema.columns\n" + //
                    "WHERE \n" + //
                    "    table_catalog = 'cukierka'\n" + //
                    "    AND table_schema NOT LIKE 'pg_%'\n" + //
                    "    AND table_schema != 'information_schema'\n" + //
                    "ORDER BY \n" + //
                    "    table_name, ordinal_position;\n" + //
                    "");
            JSONArray jsonArray = new JSONArray();

            while (rs.next()) {
                jsonArray.put(Tools.toJSONObject(rs));
            }
            return new JSONObject().put("answer", jsonArray);
        } catch (SQLException e) {
            e.printStackTrace();
            return null;
        }
    }

    public JSONObject getUserInfo(String user) {
        try {
            PreparedStatement preparedStatement = connection.prepareStatement("SELECT * FROM Users WHERE id = ?");
            preparedStatement.setString(1, user);
            ResultSet rs = preparedStatement.executeQuery();
            if (rs.next()) {
                return Tools.toJSONObject(rs);
            } else {
                return null;
            }
        } catch (SQLException e) {
            e.printStackTrace();
            return null;
        }
    }

    public JSONObject addUser(String id, String name, String email, JSONObject other) {
        try {
            PreparedStatement preparedStatement = connection
                    .prepareStatement("INSERT INTO Users (id, username, email, other) VALUES (?, ?, ?, ?);");
            preparedStatement.setString(1, id);
            preparedStatement.setString(2, name);
            preparedStatement.setString(3, email);
            preparedStatement.setString(4, other.toString());
            String sql = preparedStatement.toString() + ";";
            preparedStatement.close();
            // Брух и кринж, я чето не понял, поэтому пока затычка в java > sql > postgesql. Я потом либо разберусь, либо напишу свой компоновщик, а не этот через состояние.
            if (connection.createStatement().executeUpdate(sql) == 1) {
                return new JSONObject().put("result", "ok");
            } else {
                return new JSONObject().put("result", "error");
            }
        } catch (SQLException e) {
            e.printStackTrace();
            return null;
        }
    }

    public JSONObject addToken(String id, String user) {
        try {
            PreparedStatement preparedStatement = connection.prepareStatement("INSERT INTO Tokens VALUES (?, ?)");
            preparedStatement.setString(1, id);
            preparedStatement.setString(2, user);
            if (preparedStatement.executeUpdate() == 1) {
                return new JSONObject().put("result", "ok");
            } else {
                return new JSONObject().put("result", "error");
            }
        } catch (SQLException e) {
            e.printStackTrace();
            return null;
        }
    }
}
