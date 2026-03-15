import { createGreetModel } from "../model";
import { Button } from "@/shared/ui";

export const GreetForm = () => {
  const { name, setName, greetMsg, greet } = createGreetModel();

  return (
    <>
      <form
        class="row"
        onSubmit={(e) => {
          e.preventDefault();
          void greet();
        }}
      >
        <input
          id="greet-input"
          value={name()}
          onInput={(e) => setName(e.currentTarget.value)}
          placeholder="Enter a name..."
        />
        <Button type="submit">Greet</Button>
      </form>
      <p>{greetMsg()}</p>
    </>
  );
};

