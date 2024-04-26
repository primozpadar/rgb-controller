import { Feather } from "@expo/vector-icons";
import { Link } from "expo-router";
import { Pressable, StyleSheet, Text, View } from "react-native";

interface Props {
  showBack?: boolean;
}

export default function TopBar({ showBack }: Props) {
  return (
    <>
      <View style={styles.settingsBar}>
        {showBack ? (
          <>
            <Link href="/" asChild>
              <Pressable style={styles.btnWithText}>
                <Feather name="arrow-left" color="#ffffff" size={16} />
                <Text style={styles.btnText}>Nazaj</Text>
              </Pressable>
            </Link>
          </>
        ) : (
          <>
            <Text style={styles.title}>RGB controller</Text>
            <Link href="/settings" asChild>
              <Pressable style={styles.roundBtn}>
                <Feather name="settings" color="#ffffff" size={16} />
              </Pressable>
            </Link>
          </>
        )}
      </View>
    </>
  );
}

const styles = StyleSheet.create({
  settingsBar: {
    marginVertical: 8,
    padding: 8,
    backgroundColor: "#ffffff22",
    borderRadius: 8,
    justifyContent: "space-between",
    flexDirection: "row",
    alignItems: "center",
  },
  title: {
    fontSize: 24,
    fontWeight: "bold",
    color: "white",
  },
  roundBtn: {
    backgroundColor: "#3c2fed",
    width: 32,
    height: 32,
    justifyContent: "center",
    alignItems: "center",
    borderRadius: 6,
  },
  btnWithText: {
    height: 32,
    backgroundColor: "#3c2fed",
    borderRadius: 6,
    gap: 8,
    alignItems: "center",
    flexDirection: "row",
    paddingHorizontal: 8,
  },
  btnText: {
    color: "#ffffff",
    fontWeight: "600",
  },
});
