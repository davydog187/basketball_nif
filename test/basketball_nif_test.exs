defmodule BasketballNifTest do
  use ExUnit.Case
  doctest BasketballNif

  test "greets the world" do
    assert BasketballNif.hello() == :world
  end
end
