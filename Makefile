TARGET = glfw

ROOT_DIR = .

SRC_DIR        = $(ROOT_DIR)/src
SRC_CRATE      = $(TARGET).rc
BUILD_DIR      = $(ROOT_DIR)/lib

$(TARGET):
	@echo "Building $(TARGET)"
	@mkdir -p $(BUILD_DIR)
	@rustc $(SRC_DIR)/$(SRC_CRATE) --lib --out-dir=$(BUILD_DIR)
	@echo "Success! \o/"

all: $(TARGET)

clean:
	rm -R -f $(BUILD_DIR)