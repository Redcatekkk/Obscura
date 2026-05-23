import { cleanup, fireEvent, render, screen } from "@testing-library/react";
import { afterEach, describe, expect, it } from "vitest";
import App from "./App";

afterEach(() => {
  cleanup();
});

describe("App", () => {
  it("renders the static control deck surfaces", () => {
    render(<App />);

    expect(screen.getByLabelText("obscura deck workspace")).toBeTruthy();
    expect(screen.getByTestId("master-control")).toBeTruthy();
    expect(screen.getByTestId("function-library")).toBeTruthy();
    expect(screen.getByTestId("terminal-feed")).toBeTruthy();
    expect(screen.getByTestId("disclaimer-card")).toHaveTextContent("Simulation only");
  });

  it("filters modules and opens the config modal", async () => {
    render(<App />);

    await screen.findByTestId("function-card-f01");
    fireEvent.click(screen.getByTestId("open-config"));

    expect(screen.getByTestId("config-modal")).toHaveTextContent("Message Sniper");
  });

  it("optimistically toggles module state", async () => {
    render(<App />);

    const toggle = await screen.findByTestId("toggle-message-sniper");

    expect(toggle).toHaveAttribute("aria-pressed", "true");
    fireEvent.click(toggle);
    expect(toggle).toHaveAttribute("aria-pressed", "false");
  });

  it("toggles sim intensity with the engage button", async () => {
    render(<App />);

    const engageButton = await screen.findByTestId("engage-button");

    expect(engageButton).toHaveAttribute("aria-pressed", "true");
    expect(engageButton).toHaveTextContent("ENGAGED");

    fireEvent.click(engageButton);

    expect(engageButton).toHaveAttribute("aria-pressed", "false");
    expect(engageButton).toHaveTextContent("DISENGAGED");
  });
});
