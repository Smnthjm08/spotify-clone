import Header from "@smartgrow/ui/components/header";
import { Button } from "@smartgrow/ui/components/ui/button";
import { toast } from "sonner";

function App() {
  return (
    <main className="flex flex-col items-center justify-center gap-4">
      <Header />
      <Button onClick={() => toast.success("hello")}>Click Me</Button>
    </main>
  );
}

export default App;
