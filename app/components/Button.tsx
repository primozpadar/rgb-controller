import { Pressable, PressableProps, StyleProp, StyleSheet, Text, TextStyle, ViewStyle } from "react-native";

interface Props {
  onPress: () => void;
  children: string | string[];
  style?: ViewStyle;
  textStyle?: TextStyle
}

export default function Button(props: Props) {
  return (
    <Pressable onPress={props.onPress} style={[styles.container, props.style]}>
      <Text style={[styles.text, props.textStyle]}>{props.children}</Text>
    </Pressable>
  );
}

const styles = StyleSheet.create({
  container: {
    backgroundColor: "#3c2fed",
    borderRadius: 6,
    padding: 8,
    justifyContent: "center",
    gap: 8,
    flexDirection: "row",
    alignItems: "center",
  },
  text: {
    color: "#fff",
    fontWeight: "600",
  },
});
