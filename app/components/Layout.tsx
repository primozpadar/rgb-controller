import { PropsWithChildren } from "react";
import { View } from "react-native";
import { useSafeAreaInsets } from "react-native-safe-area-context";

export default function Layout(props: PropsWithChildren) {
  const insets = useSafeAreaInsets();

  return (
    <View
      style={{
        flex: 1,
        top: insets.top,
        bottom: insets.bottom,
        justifyContent: "center",
      }}
    >
      {props.children}
    </View>
  );
}
