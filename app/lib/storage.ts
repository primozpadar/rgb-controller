import AsyncStorage from "@react-native-async-storage/async-storage";

export type Device = {
  name: string;
  ip: string;
  channel: number;
  color: [number, number, number];
};

const DEVICES_STORAGE_KEY = "rgb-controller-devices";

export async function getDevices(): Promise<Device[]> {
  try {
    const items = await AsyncStorage.getItem(DEVICES_STORAGE_KEY);
    if (!items) return [];
    return JSON.parse(items);
  } catch (e) {
    alert("Napaka pri nalaganju naprav!");
  }
  return [];
}

export async function addDevice(device: Device) {
  try {
    const existing = await getDevices();
    await AsyncStorage.setItem(DEVICES_STORAGE_KEY, JSON.stringify([...existing, device]));
  } catch (e) {
    alert("Napaka pri dodajanju naprave!");
  }
}

export async function removeDevice(index: number) {
  try {
    const existing = await getDevices();
    const updated = existing.filter((_, i) => i !== index);
    await AsyncStorage.setItem(DEVICES_STORAGE_KEY, JSON.stringify(updated));
  } catch (e) {
    alert("Napaka pri odstranjevanju naprave!");
  }
}
