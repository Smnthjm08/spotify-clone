import { Button } from "./ui/button";

export default function Header() {
  return (
    <header className="flex h-[70px] w-full flex-shrink-0 flex-row items-center justify-between bg-gray-600 px-6">
      <h1 className="font-bold">SMARTGROW</h1>
      <div className="space-x-4">
        <Button variant={"defaultpointer"}>Request Access</Button>
        <Button variant={"defaultpointer"}>Login</Button>
      </div>
    </header>
  );
}
