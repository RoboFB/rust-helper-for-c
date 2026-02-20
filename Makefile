# **************************************************************************** #
#                                                                              #
#                                                         :::      ::::::::    #
#    Makefile                                           :+:      :+:    :+:    #
#                                                     +:+ +:+         +:+      #
#    By: rgohrig <rgohrig@student.42heilbronn.de>   +#+  +:+       +#+         #
#                                                 +#+#+#+#+#+   +#+            #
#    Created: 2026/01/26 11:27:55 by rgohrig           #+#    #+#              #
#    Updated: 2026/02/13 20:08:13 by rgohrig          ###   ########.fr        #
#                                                                              #
# **************************************************************************** #



# **************************************************************************** #
#                             C MAKEFILE
# **************************************************************************** #

# ----------------------------- GENERAL ----------------------------------------

NAME :=				miniRT

COMPILER :=			cc

DEBUG_FLAGS:=		-fsanitize=address,undefined
FAST_FLAGS :=		-march=native -O3 # Ofast is more extreme than O3 alters math stuff
CFLAGS :=			-Wall -Werror -Wextra -Wdouble-promotion -g3 $(FAST_FLAGS)
LINKER_FLAGS :=		-ffast-math -flto -Wpadded # is Wpadded used here ?
COMPILE_FLAGS :=	-MMD -MP # MMD & MD for dependencies
LIBMLX_FLAGS :=		-ldl -lglfw -pthread -lm

# -ffast -flto ARE LINKER FLAGS

DIR_SRC :=		src
SRC :=			$(patsubst $(DIR_SRC)/%,%,$(shell find $(DIR_SRC) -type f -name "*.c")) # TODO: at end fix


DIR_OBJ :=		obj
OBJ :=			$(SRC:%.c=$(DIR_OBJ)/%.o)

LIBFT_DIR :=	./libft
LIBFT :=		$(LIBFT_DIR)/libft.a

LIBMLX_DIR :=	./MLX42
LIBMLX :=		$(LIBMLX_DIR)/build/libmlx42.a


HEADERS :=		-I $(LIBFT_DIR)/include -I $(LIBMLX_DIR)/include/MLX42 -I ./include
LIBS :=			$(LIBFT) $(LIBMLX) $(LIBMLX_FLAGS)


DEPENDENCIES := $(OBJ:.o=.d)


# ----------------------------- NORMAL -----------------------------------------

# default Rule
all: lazy_robin stop $(LIBFT) $(LIBMLX) $(NAME) #TODO: rm at end lazy_robin



$(LIBFT):
	@$(MAKE) --no-print-directory -C $(LIBFT_DIR) CFLAGS="$(CFLAGS)"


$(LIBMLX):
	@git submodule update --init --recursive
# 	@cmake -DDEBUG=1 $(LIBMLX_DIR) -B $(LIBMLX_DIR)/build > /dev/null && make -C $(LIBMLX_DIR)/build -j4 > /dev/null
	@cmake -DDEBUG=1 $(LIBMLX_DIR) -B $(LIBMLX_DIR)/build && make -C $(LIBMLX_DIR)/build -j4 
	@echo "   🛠️ 🛠️ 🛠️  MLX42 compiled"

#  -DDEBUG=1 at cmake for debug infos # -DGLFW_FETCH=0 purpose ?


$(DIR_OBJ):
	@mkdir $(DIR_OBJ)

# Compilation
$(DIR_OBJ)/%.o : $(DIR_SRC)/%.c | $(DIR_OBJ)
	@mkdir -p $(dir $@)
	@$(COMPILER) $(CFLAGS) $(COMPILE_FLAGS) $(HEADERS) -o $@ -c $<
	@echo 🎇 $@

# Linking
$(NAME): $(OBJ)
	@$(COMPILER) $(CFLAGS) $(LINKER_FLAGS) -o $@ $^ $(LIBS)
	@echo "\n   🎇🎇🎇 $@   ($(CFLAGS))\n"

# ----------------------------- Dependencies -----------------------------------
-include $(DEPENDENCIES)

# ----------------------------- Clean ------------------------------------------

stop:
	@pkill -x $(NAME) > /dev/null 2>&1 && echo "🛑 stopped $(NAME)" || true

clean:
	@rm -rf $(DEPENDENCIES)
	@rm -rf $(OBJ)
	@$(MAKE) --no-print-directory -C $(LIBFT_DIR) clean
# 	@rm -rf $(LIBMLX_DIR)/build
	@echo 🧹 cleaned $(NAME) objects

fclean: clean
	@rm -rf $(NAME)
	@$(MAKE) --no-print-directory -C $(LIBFT_DIR) fclean > /dev/null
# 	@rm -rf $(LIBMLX_DIR)/build > /dev/null
	@echo 🧹🧹🧹 cleaned $(NAME)

re:
	@$(MAKE) --no-print-directory fclean
	@$(MAKE) --no-print-directory all

# ----------------------------- Debug ------------------------------------------

# debug: fclean
# debug: CFLAGS += $(DEBUG_FLAGS)
# debug: all

debug: CFLAGS += $(DEBUG_FLAGS)
debug: CFLAGS := $(filter-out $(FAST_FLAGS),$(CFLAGS))
debug:
	@$(COMPILER) $(CFLAGS) $(HEADERS) -o $(NAME) $(addprefix $(DIR_SRC)/,$(SRC)) $(LIBS)
	@echo "\n   🐞🐞🐞 DEBUG $(NAME)   ($(CFLAGS))\n"
	@./miniRT

# ----------------------------- Lazy Robin -------------------------------------

# temporary Rule to update the header file
lazy_robin:
	@awk '/ auto/ { exit } { print }' include/mini_rt.h > tmp-auto-header.h
	@echo '// auto' >> tmp-auto-header.h
	@awk '/^[a-zA-Z_][a-zA-Z0-9_ \*\t]*\([^\)]*\)[ \t]*$$/ { \
			last=$$0; \
			getline; \
			if ($$0 ~ /^\s*\{/) { \
					split(last, a, /[ \t]+/); \
					if (a[1] == "int") sub(/[ \t]+/, "\t\t\t", last); \
					else sub(/[ \t]+/, "\t\t", last); \
					print last ";"; \
			} \
	}' $(shell find $(DIR_SRC) -type f -name '*.c') | grep -v static >> tmp-auto-header.h
	@echo "\n#endif" >> tmp-auto-header.h
	@cmp -s tmp-auto-header.h include/mini_rt.h || mv tmp-auto-header.h include/mini_rt.h
	@rm -f tmp-auto-header.h

# ----------------------------- Phony ------------------------------------------

.PHONY: all clean fclean re debug stop
