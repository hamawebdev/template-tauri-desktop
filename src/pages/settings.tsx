import { platform, version } from "@tauri-apps/plugin-os";
import { useAppStore } from "@/store/use-app-store";
import { Button } from "@/components/ui/button";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";

const themes = ["light", "dark", "system"] as const;

export default function SettingsPage() {
  const theme = useAppStore((s) => s.theme);
  const setTheme = useAppStore((s) => s.setTheme);
  // plugin-os exposes platform()/version() synchronously under Tauri.
  const osInfo = `${platform()} ${version()}`;

  return (
    <div className="mx-auto flex max-w-2xl flex-col gap-6">
      <div>
        <h2 className="text-2xl font-bold tracking-tight">Settings</h2>
        <p className="text-muted-foreground">Theme and system information.</p>
      </div>

      <Card>
        <CardHeader>
          <CardTitle>Appearance</CardTitle>
          <CardDescription>Persisted via Zustand.</CardDescription>
        </CardHeader>
        <CardContent className="flex gap-2">
          {themes.map((t) => (
            <Button
              key={t}
              variant={theme === t ? "default" : "outline"}
              onClick={() => setTheme(t)}
              className="capitalize"
            >
              {t}
            </Button>
          ))}
        </CardContent>
      </Card>

      <Card>
        <CardHeader>
          <CardTitle>System</CardTitle>
          <CardDescription>Read via @tauri-apps/plugin-os.</CardDescription>
        </CardHeader>
        <CardContent>
          <p className="text-sm">{osInfo || "…"}</p>
        </CardContent>
      </Card>
    </div>
  );
}
