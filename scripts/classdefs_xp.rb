# Ruby class definitions of rpg maker xp data structures
class Color
  attr_accessor :red, :green, :blue, :alpha

  def _dump(limit)
    [@red, @green, @blue, @alpha].pack("d4")
  end

  def self._load(data)
    obj = Color.new
    obj.red, obj.green, obj.blue, obj.alpha = *data.unpack("d4")
    obj
  end
end

class Table
  attr_accessor :num_of_dimensions, :xsize, :ysize, :zsize, :num_of_elements, :elements

  def _dump(limit)
    [@num_of_dimensions, @xsize, @ysize, @zsize, @num_of_elements, @elements].flatten.pack("VVVVVv*")
  end

  def self._load(data)
    obj = self.new
    obj.num_of_dimensions, obj.xsize, obj.ysize, obj.zsize, obj.num_of_elements, *obj.elements = *data.unpack("VVVVVv*")
    obj
  end
end

class Tone
  attr_accessor :red, :green, :blue, :gray

  def _dump(limit)
    [@red, @green, @blue, @gray].pack("d4")
  end

  def self._load(data)
    obj = Tone.new
    obj.red, obj.green, obj.blue, obj.gray = *data.unpack("d4")
    obj
  end
end

module RPG
  class Event
    class Page
      class Condition
      end

      class Graphic
      end
    end
  end

  class EventCommand
  end

  class MoveRoute
  end

  class MoveCommand
  end

  class Map
  end

  class MapInfo
  end

  class AudioFile
  end

  class System
    class Words
    end

    class TestBattler
    end
  end

  class CommonEvent
  end

  class Tileset
  end

  class State
  end

  class Animation
    class Frame
    end

    class Timing
    end
  end

  class Class
    class Learning
    end
  end

  class Actor
  end

  class Skill
  end

  class Item
  end

  class Weapon
  end

  class Armor
  end

  class Enemy
    class Action
    end
  end

  class Troop
    class Member
    end

    class Page
      class Condition
      end
    end
  end
end